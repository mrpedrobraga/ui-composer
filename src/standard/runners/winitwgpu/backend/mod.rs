//! A Backend that uses Winit to create Windowing and WGPU to render to the window.

use crate::app::backend::AppContext;
use crate::standard::runners::wgpu::backend::{WgpuBackend, WgpuResources};
use crate::standard::runners::wgpu::pipeline::graphics::OrchestraRenderer;
use crate::standard::runners::wgpu::pipeline::{UIContext, WgpuRenderers, text::TextRenderer};
use crate::state::process::Pollable;
use pin_project::pin_project;
use std::sync::Mutex;
use winit::event::{DeviceEvent, DeviceId};
use winit::event_loop::{ControlFlow, EventLoop};
use {
    super::window::WindowRenderTarget,
    crate::app::backend::{Runner, futures::AsyncExecutor},
    futures_signals::signal::{SignalExt, SignalFuture},
    std::{
        ops::DerefMut,
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
    },
    wgpu::{MemoryHints, TextureFormat},
    winit::{
        application::ApplicationHandler,
        event::WindowEvent,
        event_loop::ActiveEventLoop,
        window::{WindowAttributes, WindowId},
    },
};

pub mod implementations;


#[pin_project(project=WithWinitProj)]
pub struct WinitWgpuBackend<A: Node>(WgpuBackend<A>);

/// The [`ApplicationHandler`] that sits between [`winit`]
/// and UI Composer.
pub struct WinitWGPUApplicationHandler<A: Node> {
    tree_descriptor: Option<A>,
    backend: Option<SharedWWBackend<A>>,
}


impl<A> Runner for WinitWgpuBackend<A>
where
    A: Node<Output: Pollable<AppContext>> + 'static,
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
    A: Node + Send + 'static,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.backend.is_none() {
            let (backend, processor) = futures::executor::block_on(WinitWgpuBackend::<_>::create(
                event_loop,
                self.tree_descriptor
                    .take()
                    .expect("Failed to create WinitWgpu app."),
            ));

            // TODO: Allow the ui to make the window close.
            let _join_handle = std::thread::spawn(|| {
                futures::executor::block_on(processor);
            });

            self.backend = Some(backend)
        }

        if let Some(backend) = &mut self.backend {
            let mut backend = backend.lock().unwrap();
            backend.handle_resumed();
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(backend) = &mut self.backend {
            let mut backend = backend.lock().unwrap();
            backend.handle_window_event(window_id, event);
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

impl<A: Node> WgpuBackend<A> {
    // Little hack to get a "dummy" texture format.
    // This should go unused hopefully!
    #[allow(unused)]
    fn get_dummy_texture_format(
        event_loop: &ActiveEventLoop,
        instance: &wgpu::Instance,
        device: &wgpu::Device,
        adapter: &wgpu::Adapter,
    ) -> TextureFormat {
        let dummy_window = event_loop
            .create_window(WindowAttributes::default())
            .expect("Failure to create dummy window");
        let dummy_surface = instance
            .create_surface(dummy_window)
            .expect("Failure to get dummy window surface.");
        dummy_surface.configure(
            device,
            &dummy_surface
                .get_default_config(adapter, 20, 20)
                .expect("No valid config between this surface and the adapter."),
        );
        let dummy_texture = dummy_surface
            .get_current_texture()
            .expect("Failure to get dummy texture.");

        dummy_texture.texture.format()
    }
}

impl<A> WinitWgpuBackend<A>
where
    A: Node<Output: Pollable<AppContext>> + 'static,
{
    async fn create(
        event_loop: &ActiveEventLoop,
        tree_descriptor: A,
    ) -> (Arc<Mutex<Self>>, SignalFuture<AsyncExecutor<Self>>) {
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

        // TODO: Get the texture format from a render target and not guess it.
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

        let renderers = WgpuRenderers {
            graphics_renderer: graphics_pipeline,
            text_renderer: text_pipeline,
        };

        let gpu_resources = WgpuResources {
            instance,
            device,
            queue,
            adapter,
        };

        let tree = tree_descriptor.reify(event_loop, &gpu_resources, renderers);

        let backend = WgpuBackend {
            tree: Arc::new(Mutex::new(tree)),
            gpu_resources,
        };

        // This render engine will be shared between your application (so that it can receive OS events)
        // and a reactor processor in another thread (so that it can process its own `Future`s and `Signal`s)
        let backend = Arc::new(Mutex::new(WinitWgpuBackend(backend)));

        // TODO: Process the render engine's processors and reactors on another thread.
        // It needs not be an other thread, just a different execution context.
        let backend_clone = backend.clone();

        let backend_processor = AsyncExecutor::new(backend_clone).to_future();

        // I should perhaps return a (RenderEngine, impl Future) here!
        // The problem of course is that winit does not play well with async Rust yet :-(
        (backend, backend_processor)
    }

    fn handle_resumed(&mut self) {
        self.0.tree.lock().unwrap().setup(&self.0.gpu_resources);
    }

    /// Forwards window events for the engine tree to process.
    fn handle_window_event(&mut self, window_id: WindowId, event: WindowEvent) {
        self.0
            .tree
            .lock()
            .unwrap()
            .handle_window_event(&mut self.0.gpu_resources, window_id, event)
    }
}

/// Trait that represents a descriptor main node of the app tree.
/// These nodes are used for creating windows and processes and rendering contexts.
///
/// TODO: Delete this and use [BuildingBlock] instead.
#[diagnostic::on_unimplemented(message = "This value is not an app Node.")]
pub trait Node: Send {
    /// The type this node descriptor generates when Output.
    type Output: NodeRe;
    fn reify(
        self,
        event_loop: &ActiveEventLoop,
        gpu_resources: &WgpuResources,
        renderers: WgpuRenderers,
    ) -> Self::Output;
}

/// A main node in the app tree.
pub trait NodeRe: Pollable<AppContext> {
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
type SharedWWBackend<A: Node<Output: Pollable<UIContext>>> = Arc<Mutex<WinitWgpuBackend<A>>>;
