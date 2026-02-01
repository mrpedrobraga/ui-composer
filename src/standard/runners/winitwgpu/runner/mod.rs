//! A Backend that uses Winit to create Windowing and WGPU to render to the window.

use crate::app::backend::AppContext;
use crate::standard::runners::wgpu::backend::{WgpuBackend, WgpuResources};
use crate::standard::runners::wgpu::pipeline::graphics::OrchestraRenderer;
use crate::standard::runners::wgpu::pipeline::{text::TextRenderer, UIContext, WgpuRenderers};
use crate::state::process::Pollable;
use pin_project::pin_project;
use std::sync::Mutex;
use wgpu::{Adapter, Device, Queue};
use winit::event::{DeviceEvent, DeviceId};
use winit::event_loop::{ControlFlow, EventLoop};
use {
    super::window::WindowRenderTarget,
    crate::app::backend::{futures::AsyncExecutor, Runner},
    futures_signals::signal::{SignalExt, SignalFuture},
    std::{
        ops::DerefMut,
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
    },
    wgpu::{MemoryHints, TextureFormat},
    winit::{
        application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
        window::WindowId,
    },
};

pub mod implementations;

#[pin_project(project=WithWinitProj)]
pub struct WinitWgpuRunner<A: EReify>(WgpuBackend<A>);

/// The [`ApplicationHandler`] that sits between [`winit`]
/// and UI Composer.
pub struct WinitWGPUApplicationHandler<A: EReify> {
    tree_descriptor: Option<A>,
    backend: Option<SharedWWBackend<A>>,
}

/// Trait that represents a descriptor main node of the app tree.
/// These nodes are used for creating windows and processes and rendering contexts.
///
/// TODO: Delete this and use [BuildingBlock] instead.
#[diagnostic::on_unimplemented(message = "This value is not an app Node.")]
pub trait EReify: Send {
    /// The type this node descriptor generates when Output.
    type Output: Element;
    fn reify(
        self,
        event_loop: &ActiveEventLoop,
        gpu_resources: &WgpuResources,
        renderers: WgpuRenderers,
    ) -> Self::Output;
}

/// A main node in the app tree.
pub trait Element: Pollable<AppContext> {
    fn setup(&mut self, gpu_resources: &WgpuResources);

    /// Handles an event and cascades it down its children.
    fn handle_window_event(
        &mut self,
        gpu_resources: &mut WgpuResources,
        window_id: WindowId,
        event: WindowEvent,
    );
}

#[expect(type_alias_bounds)]
type SharedWWBackend<A: EReify<Output: Pollable<UIContext>>> = Arc<Mutex<WinitWgpuRunner<A>>>;

impl<A> Runner for WinitWgpuRunner<A>
where
    A: EReify<Output: Pollable<AppContext>> + 'static,
{
    type App = A;

    /// This function *must* be called on the main thread, because of winit.
    fn run(node_tree: Self::App) {
        let event_loop = EventLoop::builder().build().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop
            .run_app(&mut WinitWGPUApplicationHandler::<A> {
                tree_descriptor: Some(node_tree),
                backend: None,
            })
            .unwrap();
    }

    fn process(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &mut AppContext,
    ) -> Poll<Option<()>> {
        let WithWinitProj(backend) = self.project();

        let mut tree = backend.tree.lock().unwrap();
        let tree = tree.deref_mut();
        let tree_pin = unsafe { Pin::new_unchecked(tree) };

        tree_pin.poll(cx, resources)
    }
}

impl<A> ApplicationHandler for WinitWGPUApplicationHandler<A>
where
    A: EReify + Send + 'static,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.backend.is_none() {
            let (backend, processor) = futures::executor::block_on(create_winit_runner(
                event_loop,
                self.tree_descriptor
                    .take()
                    .expect("Failed to create WinitWgpu app."),
            ));

            // TODO: Allow the ui to make the window close.
            let _join_handle = std::thread::spawn(|| {
                futures::executor::block_on(processor.to_future());
            });

            self.backend = Some(backend)
        }

        if let Some(backend) = &mut self.backend {
            let backend = backend.lock().unwrap();
            backend
                .0
                .tree
                .lock()
                .unwrap()
                .setup(&backend.0.gpu_resources);
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(backend) = &mut self.backend {
            let backend = &mut backend.lock().unwrap().0;
            backend.tree.lock().unwrap().handle_window_event(
                &mut backend.gpu_resources,
                window_id,
                event,
            )
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        _event: DeviceEvent,
    ) {
        // TODO: Handle device events...
    }
}

async fn create_winit_runner<A>(
    event_loop: &ActiveEventLoop,
    tree_descriptor: A,
) -> (
    Arc<Mutex<WinitWgpuRunner<A>>>,
    AsyncExecutor<WinitWgpuRunner<A>>,
)
where
    A: EReify<Output: Pollable<AppContext>> + 'static,
{
    let gpu_resources = create_gpu_resources().await;
    let renderers = create_renderers(
        &gpu_resources.adapter,
        &gpu_resources.device,
        &gpu_resources.queue,
    );
    let tree = tree_descriptor.reify(event_loop, &gpu_resources, renderers);

    let backend = WgpuBackend {
        tree: Arc::new(Mutex::new(tree)),
        gpu_resources,
    };

    create_backend_effect_executors(backend)
}

async fn create_gpu_resources() -> WgpuResources {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                .using_resolution(adapter.limits()),
            memory_hints: MemoryHints::default(),
            // TODO: Add something here in debug mode.
            trace: Default::default(),
        })
        .await
        .unwrap();

    WgpuResources {
        instance,
        device,
        queue,
        adapter,
    }
}

fn create_renderers(adapter: &Adapter, device: &Device, queue: &Queue) -> WgpuRenderers {
    // TODO: Get this from a render target, do not guess it.
    let dummy_format = TextureFormat::Bgra8UnormSrgb; //;get_dummy_texture_format(event_loop, &instance, &device, &adapter);

    let render_target_formats = &[Some(wgpu::ColorTargetState {
        format: dummy_format,
        blend: Some(wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent::OVER,
        }),
        write_mask: wgpu::ColorWrites::ALL,
    })];

    let graphics_pipeline = OrchestraRenderer::singleton::<WindowRenderTarget>(
        &adapter,
        &device,
        &queue,
        render_target_formats,
    );

    let text_pipeline = TextRenderer::singleton::<WindowRenderTarget>(
        &adapter,
        &device,
        &queue,
        render_target_formats,
    );

    WgpuRenderers {
        graphics_renderer: graphics_pipeline,
        text_renderer: text_pipeline,
    }
}

fn create_backend_effect_executors<A>(
    backend: WgpuBackend<A>,
) -> (
    Arc<Mutex<WinitWgpuRunner<A>>>,
    AsyncExecutor<WinitWgpuRunner<A>>,
)
where
    A: EReify<Output: Pollable<AppContext>> + 'static,
{
    // This render engine will be shared between your application (so that it can receive OS events)
    // and a reactor processor in another thread (so that it can process its own `Future`s and `Signal`s)
    let backend = Arc::new(Mutex::new(WinitWgpuRunner(backend)));

    // TODO: Process the render engine's processors and reactors on another thread.
    // It needs not be an other thread, just a different execution context.
    let backend_clone = backend.clone();

    let backend_processor = AsyncExecutor::new(backend_clone);

    // I should perhaps return a (RenderEngine, impl Future) here!
    // The problem of course is that winit does not play well with async Rust yet :-(
    (backend, backend_processor)
}
