//! A Backend that uses Winit to create Windowing and WGPU to render to the window.

use crate::app::primitives::{Primitive, Processor};
use crate::wgpu::backend::{Resources, WGPUBackend};
use crate::wgpu::pipeline::graphics::OrchestraRenderer;
use crate::wgpu::pipeline::{text::GlyphonTextRenderer, Renderers};
use pin_project::pin_project;
use std::marker::PhantomData;
use std::sync::Mutex;
use winit::event::{DeviceEvent, DeviceId};
use {
    super::window::WindowRenderTarget,
    crate::{
        app::backend::{Backend, BackendProcessExecutor},
        winit::WinitBackend,
    },
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

/// Trait that represents a descriptor main node of the app tree.
/// These nodes are used for creating windows and processes and rendering contexts.
#[diagnostic::on_unimplemented(message = "This value is not a proper node descriptor.")]
pub trait NodeDescriptor: Send {
    /// The type this node descriptor generates when reified.
    type Reified: Node;
    fn reify(
        self,
        event_loop: &ActiveEventLoop,
        gpu_resources: &Resources,
        renderers: &mut Renderers,
    ) -> Self::Reified;
}

/// A main node in the app tree.
pub trait Node: Primitive {
    fn setup(&mut self, gpu_resources: &Resources);

    /// Handles an event and cascades it down its children.
    fn handle_window_event(
        &mut self,
        gpu_resources: &mut Resources,
        pipelines: &mut Renderers,
        window_id: WindowId,
        event: WindowEvent,
    );
}

#[expect(type_alias_bounds)]
type SharedWWBackend<A: NodeDescriptor> = Arc<Mutex<WithWinit<WGPUBackend<A::Reified, A>>>>;

/// The [`ApplicationHandler`] that sits between [`winit`]
/// and UI Composer.
pub struct WinitWGPUApplicationHandler<A: NodeDescriptor> {
    tree_descriptor: Option<A>,
    backend: Option<SharedWWBackend<A>>,
}

#[pin_project(project=WithWinitProj)]
pub struct WithWinit<A>(A);

impl<A: NodeDescriptor + 'static> Backend for WithWinit<WGPUBackend<A::Reified, A>> {
    type Tree = A;

    /// This function *must* be called on the main thread, because of winit.
    fn run(node_tree: Self::Tree) {
        let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        event_loop
            .run_app(&mut WinitWGPUApplicationHandler::<A> {
                tree_descriptor: Some(node_tree),
                backend: None,
            })
            .unwrap();
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        let WithWinitProj(backend) = self.project();

        let mut tree = backend.tree.lock().unwrap();
        let tree = tree.deref_mut();
        let tree_pin = unsafe { Pin::new_unchecked(tree) };

        tree_pin.poll(cx)
    }
}

impl<A: NodeDescriptor + 'static> ApplicationHandler for WinitWGPUApplicationHandler<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.backend.is_none() {
            let (backend, processor) =
                futures::executor::block_on(WithWinit::<WGPUBackend<_, _>>::create(
                    event_loop,
                    self.tree_descriptor
                        .take()
                        .expect("Failed to create WinitWgpu app."),
                ));

            std::thread::spawn(|| {
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

impl<A: NodeDescriptor + Send> WGPUBackend<A::Reified, A> {
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

impl<A: NodeDescriptor + Send + 'static> WinitBackend for WithWinit<WGPUBackend<A::Reified, A>> {
    type NodeTreeDescriptorType = A;

    async fn create(
        event_loop: &ActiveEventLoop,
        tree_descriptor: A,
    ) -> (Arc<Mutex<Self>>, SignalFuture<BackendProcessExecutor<Self>>) {
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

        let text_pipeline = GlyphonTextRenderer::singleton::<WindowRenderTarget>(
            &adapter,
            &device,
            &queue,
            render_target_formats,
        );

        let mut renderers = Renderers {
            graphics_renderer: graphics_pipeline,
            text_renderer: text_pipeline,
        };

        let gpu_resources = Resources {
            instance,
            device,
            queue,
            adapter,
        };

        let tree = tree_descriptor.reify(event_loop, &gpu_resources, &mut renderers);

        let backend = WGPUBackend {
            tree: Arc::new(Mutex::new(tree)),
            gpu_resources,
            renderers,
            _descriptor: PhantomData,
        };

        // This render engine will be shared between your application (so that it can receive OS events)
        // and a reactor processor in another thread (so that it can process its own `Future`s and `Signal`s)
        let backend = Arc::new(Mutex::new(WithWinit(backend)));

        // TODO: Process the render engine's processors and reactors on another thread.
        // It needs not be an other thread, just a different execution context.
        let backend_clone = backend.clone();

        let backend_processor = BackendProcessExecutor::new(backend_clone).to_future();

        // I should perhaps return a (RenderEngine, impl Future) here!
        // The problem of course is that winit does not play well with async Rust yet :-(
        (backend, backend_processor)
    }

    fn handle_resumed(&mut self) {
        self.0.tree.lock().unwrap().setup(&self.0.gpu_resources);
    }

    /// Forwards window events for the engine tree to process.
    fn handle_window_event(&mut self, window_id: WindowId, event: WindowEvent) {
        self.0.tree.lock().unwrap().handle_window_event(
            &mut self.0.gpu_resources,
            &mut self.0.renderers,
            window_id,
            event,
        )
    }
}
