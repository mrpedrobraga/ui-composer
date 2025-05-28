//! A Backend that uses Winit to create Windowing and WGPU to render to the window.

use {
    super::{pipeline::graphics::OrchestraRenderer, window::WindowRenderTarget},
    crate::{
        app::backend::Backend,
        winit::WinitBackend,
        winitwgpu::pipeline::{text::GlyphonTextRenderer, Renderers},
    },
    futures_signals::signal::{Signal, SignalExt, SignalFuture},
    pin_project::pin_project,
    std::{
        ops::DerefMut,
        pin::Pin,
        sync::{Arc, Mutex},
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

/// A backend that can render our application to the GPU as well as forward interactive events to the app.
#[pin_project(project=WinitWGPUBackendProj)]
pub struct WinitWGPUBackend<N: NodeDescriptor> {
    /// The node of the UI tree containing the entirety of the app, UI and behaviour.
    #[pin]
    pub tree: Arc<Mutex<N::Reified>>,
    pub gpu_resources: Resources,
    pub renderers: Renderers,
}

/// The collection of resources the GPU backends use to
/// interact with the GPU.
pub struct Resources {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
}

/// The [`ApplicationHandler`] that sits between [`winit`]
/// and UI Composer.
pub struct WinitWGPUApplicationHandler<A: NodeDescriptor> {
    node_tree_descriptor_opt: Option<A>,
    backend: Option<Arc<Mutex<WinitWGPUBackend<A>>>>,
}

impl<N: NodeDescriptor + 'static> Backend for WinitWGPUBackend<N> {
    type Event = winit::event::WindowEvent;

    type Tree = N;

    /// This function *must* be called on the main thread, because of winit.
    fn run(node_tree: Self::Tree) {
        let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        event_loop
            .run_app(&mut WinitWGPUApplicationHandler::<N> {
                node_tree_descriptor_opt: Some(node_tree),
                backend: None,
            })
            .unwrap();
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<std::option::Option<()>> {
        let WinitWGPUBackendProj { tree, .. } = self.project();

        let mut tree = tree.lock().expect("Failed to lock tree for polling");
        let tree = tree.deref_mut();
        let tree_pin = unsafe { Pin::new_unchecked(tree) };

        tree_pin.poll_processors(cx)
    }
}

impl<A: NodeDescriptor + 'static> ApplicationHandler for WinitWGPUApplicationHandler<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.backend.is_none() {
            let (backend, processor) = futures::executor::block_on(WinitWGPUBackend::create(
                event_loop,
                self.node_tree_descriptor_opt
                    .take()
                    .expect("Failed to create WinitWgpu app."),
            ));

            std::thread::spawn(|| {
                futures::executor::block_on(processor);
            });

            self.backend = Some(backend)
        }

        if let Some(backend) = &mut self.backend {
            let mut backend = backend
                .lock()
                .expect("Could not lock Render Engine to pump resumed event.");
            backend.handle_resumed();
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(backend) = &mut self.backend {
            let mut backend = backend
                .lock()
                .expect("Could not lock Render Engine to pump window event");
            backend.handle_window_event(window_id, event);
        }
    }
}

impl<E: NodeDescriptor + Send> WinitWGPUBackend<E> {
    // Little hack to get a "dummy" texture format.
    // This should go unused hopefully!
    #[allow(unused)]
    fn get_dummy_texture_format(
        event_loop: &ActiveEventLoop,
        instance: &wgpu::Instance,
        device: &wgpu::Device,
        adapter: &wgpu::Adapter,
    ) -> wgpu::TextureFormat {
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

impl<Nd: NodeDescriptor + Send + 'static> WinitBackend for WinitWGPUBackend<Nd> {
    type NodeTreeDescriptorType = Nd;

    async fn create(
        event_loop: &ActiveEventLoop,
        tree_descriptor: Nd,
    ) -> (
        Arc<Mutex<Self>>,
        SignalFuture<BackendProcessExecutor<WinitWGPUBackend<Nd>>>,
    ) {
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
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: MemoryHints::default(),
                },
                None,
            )
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

        let node_tree = tree_descriptor.reify(event_loop, &gpu_resources, &mut renderers);

        let backend = Self {
            tree: Arc::new(Mutex::new(node_tree)),
            gpu_resources,
            renderers,
        };

        // This render engine will be shared between your application (so that it can receive OS events)
        // and a reactor processor in another thread (so that it can process its own `Future`s and `Signal`s)
        let backend = Arc::new(Mutex::new(backend));

        // TODO: Process the render engine's processors and reactors on another thread.
        // It needs not be an other thread, just a different execution context.
        let backend_clone = backend.clone();

        let backend_processor = BackendProcessExecutor::new(backend_clone).to_future();

        // I should perhaps return a (RenderEngine, impl Future) here!
        // The problem of course is that winit does not play well with async Rust yet :-(
        (backend, backend_processor)
    }

    fn handle_resumed(&mut self) {
        match self.tree.lock() {
            Ok(mut engine_tree) => {
                engine_tree.setup(&self.gpu_resources);
            }
            Err(_) => unimplemented!("Could not lock mutex for handling resumed event!"),
        }
    }

    /// Forwards window events for the engine tree to process.
    fn handle_window_event(&mut self, window_id: WindowId, event: WindowEvent) {
        match self.tree.lock() {
            Ok(mut engine_tree) => engine_tree.handle_window_event(
                &mut self.gpu_resources,
                &mut self.renderers,
                window_id,
                event,
            ),
            Err(_) => unimplemented!("Could not unlock mutex for handling event!"),
        };
    }
}

/// Trait that represents a descriptor main node of the app tree.
/// These nodes are used for creating windows and processes and rendering contexts.
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
pub trait Node: Send {
    fn setup(&mut self, gpu_resources: &Resources);

    /// Handles an event and cascades it down its children.
    fn handle_window_event(
        &mut self,
        gpu_resources: &mut Resources,
        pipelines: &mut Renderers,
        window_id: WindowId,
        event: WindowEvent,
    );

    /// Polls underlying processors: `Future`s and `Signal`s within the app.
    /// This should advance animations, async processes and reactivity.
    fn poll_processors(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>>;
}

impl<A, B> NodeDescriptor for (A, B)
where
    A: NodeDescriptor,
    B: NodeDescriptor,
{
    type Reified = (A::Reified, B::Reified);

    fn reify(
        self,
        event_loop: &ActiveEventLoop,
        gpu_resources: &Resources,
        renderers: &mut Renderers,
    ) -> Self::Reified {
        (
            self.0.reify(event_loop, gpu_resources, renderers),
            self.1.reify(event_loop, gpu_resources, renderers),
        )
    }
}

impl<A, B> Node for (A, B)
where
    A: Node,
    B: Node,
{
    fn setup(&mut self, gpu_resources: &Resources) {
        self.0.setup(gpu_resources);
        self.1.setup(gpu_resources);
    }

    fn handle_window_event(
        &mut self,
        gpu_resources: &mut Resources,
        pipelines: &mut Renderers,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        self.0
            .handle_window_event(gpu_resources, pipelines, window_id, event.clone());
        self.1
            .handle_window_event(gpu_resources, pipelines, window_id, event);
    }

    fn poll_processors(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>> {
        let (pinned_a, pinned_b) = {
            let mut_ref = unsafe { self.get_unchecked_mut() };
            let (ref mut a, ref mut b) = mut_ref;

            let a = unsafe { Pin::new_unchecked(a) };
            let b = unsafe { Pin::new_unchecked(b) };

            (a, b)
        };

        let poll_a = pinned_a.poll_processors(cx);
        let poll_b = pinned_b.poll_processors(cx);

        crate::state::signal_ext::coalesce_polls(poll_a, poll_b)
    }
}

/// A futures-based construct that polls the engine's processes.
#[pin_project(project=BackendProcessExecutorProj)]
pub struct BackendProcessExecutor<E: Backend> {
    #[pin]
    backend: Arc<Mutex<E>>,
}

impl<E: Backend> BackendProcessExecutor<E> {
    pub fn new(backend: Arc<Mutex<E>>) -> Self {
        BackendProcessExecutor { backend }
    }
}

impl<E: Backend> Signal for BackendProcessExecutor<E> {
    type Item = ();

    fn poll_change(self: std::pin::Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let BackendProcessExecutorProj { backend } = self.project();

        let mut backend = backend.lock().expect("Failed to lock ui for polling");
        let backend = backend.deref_mut();
        let backend = unsafe { Pin::new_unchecked(backend) };

        backend.poll_processors(cx)
    }
}
