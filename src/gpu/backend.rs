use futures::{FutureExt, StreamExt};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use pin_project::pin_project;
use pollster::FutureExt as _;
use std::{
    borrow::BorrowMut,
    collections::HashSet,
    marker::PhantomData,
    ops::DerefMut,
    pin::Pin,
    sync::{Arc, Mutex, RwLock},
    task::{Context, Poll},
};
use vek::Extent2;
use wgpu::{MemoryHints, TextureFormat};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

use crate::ui::node::{UIEvent, UINode, UINodeDescriptor};

use super::{
    pipeline::{orchestra_render_pipeline::OrchestraRenderPipeline, GPURenderPipeline},
    render_target::{self, GPURenderTarget},
    window::{WindowNode, WindowNodeDescriptor, WindowRenderTarget},
};

/// The layer of the application that stands between the app and the outside world.
pub trait Backend {
    /// The type used for UI Events.
    type EventType;

    type NodeTreeType: NodeDescriptor + 'static;

    /// Blocking function that runs the application.
    fn run(node_tree: Self::NodeTreeType);

    /// Polls the `Futures` and `Signals` from the node tree.
    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>>;
}

pub struct GPUResources {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,

    pub main_pipeline: OrchestraRenderPipeline,
}

pub struct RunningApp<A: NodeDescriptor> {
    node_tree_descriptor_opt: Option<A>,
    backend: Option<Arc<Mutex<WinitWGPUBackend<A>>>>,
}

pub trait WinitBackend: Backend + Send {
    type NodeTreeDescriptorType: NodeDescriptor;
    fn create(
        event_loop: &ActiveEventLoop,
        tree_descriptor: Self::NodeTreeDescriptorType,
    ) -> Arc<Mutex<Self>>;
    fn handle_resumed(&mut self);
    fn handle_window_event(&mut self, window_id: WindowId, event: UIEvent);
}

/// An engine that can render our application to the GPU as well as forward interactive events to the app.
#[pin_project(project=UIEngineProj)]
pub struct WinitWGPUBackend<Nd: NodeDescriptor> {
    /// The node of the UI tree containing the entirety of the app, UI and behaviour.
    #[pin]
    pub node_tree: Arc<Mutex<Nd::ReifiedType>>,
    pub gpu_resources: GPUResources,
}

impl<Nd: NodeDescriptor + 'static> Backend for WinitWGPUBackend<Nd> {
    type EventType = winit::event::WindowEvent;

    type NodeTreeType = Nd;

    /// This function *must* be called on the main thread, because of winit.
    fn run(node_tree: Self::NodeTreeType) {
        let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        event_loop
            .run_app(&mut RunningApp::<Nd> {
                node_tree_descriptor_opt: Some(node_tree),
                backend: None,
            })
            .unwrap();
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        let UIEngineProj {
            node_tree,
            gpu_resources,
        } = self.project();

        let mut engine_tree = node_tree.lock().expect("Failed to lock tree for polling");
        let engine_tree = engine_tree.deref_mut();
        let engine_tree_pin = unsafe { Pin::new_unchecked(engine_tree) };

        engine_tree_pin.poll_processors(cx)
    }
}

impl<A: NodeDescriptor + 'static> ApplicationHandler for RunningApp<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let None = self.backend {
            self.backend = Some(WinitWGPUBackend::create(
                event_loop,
                self.node_tree_descriptor_opt
                    .take()
                    .expect("Failed to create WinitWgpu app."),
            ))
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
        event_loop: &ActiveEventLoop,
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

impl<'engine: 'static, E: NodeDescriptor + Send + 'engine> WinitWGPUBackend<E> {
    // Little hack to get a "dummy" texture format.
    // This should go unused hopefully!
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
        let dummy_format = dummy_texture.texture.format();
        dummy_format
    }
}

impl<Nd: NodeDescriptor + Send + 'static> WinitBackend for WinitWGPUBackend<Nd> {
    type NodeTreeDescriptorType = Nd;

    fn create(event_loop: &ActiveEventLoop, tree_descriptor: Nd) -> Arc<Mutex<Self>> {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .block_on()
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
            .block_on()
            .unwrap();

        // TODO: Get the texture format from a render target and not guess it.
        let dummy_format = TextureFormat::Bgra8UnormSrgb; //;get_dummy_texture_format(event_loop, &instance, &device, &adapter);
        let main_pipeline = OrchestraRenderPipeline::singleton::<WindowRenderTarget>(
            &adapter,
            &device,
            &queue,
            &[Some(dummy_format.into())],
        );

        let gpu_resources = GPUResources {
            instance,
            device,
            queue,
            adapter,
            main_pipeline,
        };

        let node_tree = tree_descriptor.reify(event_loop, &gpu_resources);

        let mut backend = Self {
            node_tree: Arc::new(Mutex::new(node_tree)),
            gpu_resources,
        };

        // This render engine will be shared between your application (so that it can receive OS events)
        // and a reactor processor in another thread (so that it can process its own `Future`s and `Signal`s)
        let backend = Arc::new(Mutex::new(backend));

        // TODO: Process the render engine's processors and reactors on another thread.
        // It needs not be an other thread, just a different execution context.
        let backend_clone = backend.clone();

        std::thread::spawn(|| {
            pollster::block_on(SignalProcessor::new(backend_clone).to_future());
            println!("Finished blocking.")
        });

        // I should perhaps return a (RenderEngine, impl Future) here!
        // The problem of course is that winit does not play well with async Rust yet :-(
        return backend;
    }

    fn handle_resumed(&mut self) {
        match self.node_tree.lock() {
            Ok(mut engine_tree) => {
                engine_tree.setup(&self.gpu_resources);
            }
            Err(_) => unimplemented!("Could not lock mutex for handling resumed event!"),
        }
    }

    /// Forwards window events for the engine tree to process.
    fn handle_window_event(&mut self, window_id: WindowId, event: UIEvent) {
        match self.node_tree.lock() {
            Ok(mut engine_tree) => {
                engine_tree.handle_window_event(&self.gpu_resources, window_id, event)
            }
            Err(_) => unimplemented!("Could not unlock mutex for handling event!"),
        };
    }
}

/// Trait that represents a descriptor main node of the engine tree.
/// These nodes are used for creating windows and processes and rendering contexts.
pub trait NodeDescriptor: Send {
    /// The type this node descriptor generates when reified.
    type ReifiedType: Node;
    fn reify(self, event_loop: &ActiveEventLoop, gpu_resources: &GPUResources)
        -> Self::ReifiedType;
}

/// A main node in the engine tree.
pub trait Node: Send {
    fn setup(&mut self, gpu_resources: &GPUResources);

    /// Handles an event by broadcasting it around.
    fn handle_window_event(
        &mut self,
        gpu_resources: &GPUResources,
        window_id: WindowId,
        event: UIEvent,
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
    type ReifiedType = (A::ReifiedType, B::ReifiedType);

    fn reify(
        self,
        event_loop: &ActiveEventLoop,
        gpu_resources: &GPUResources,
    ) -> Self::ReifiedType {
        (
            self.0.reify(event_loop, gpu_resources),
            self.1.reify(event_loop, gpu_resources),
        )
    }
}

impl<A, B> Node for (A, B)
where
    A: Node,
    B: Node,
{
    fn setup(&mut self, gpu_resources: &GPUResources) {
        self.0.setup(gpu_resources);
        self.1.setup(gpu_resources);
    }

    fn handle_window_event(
        &mut self,
        gpu_resources: &GPUResources,
        window_id: WindowId,
        event: UIEvent,
    ) {
        let a_handled = self
            .0
            .handle_window_event(gpu_resources, window_id, event.clone());
        let b_handled = self.1.handle_window_event(gpu_resources, window_id, event);
    }

    fn poll_processors(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>> {
        let (pinned_a, pinned_b) = {
            let mut mut_ref = unsafe { self.get_unchecked_mut() };
            let (ref mut a, ref mut b) = mut_ref;

            let a = unsafe { Pin::new_unchecked(a) };
            let b = unsafe { Pin::new_unchecked(b) };

            (a, b)
        };

        let poll_a = pinned_a.poll_processors(cx);
        let poll_b = pinned_b.poll_processors(cx);

        crate::prelude::coalesce_polls(poll_a, poll_b)
    }
}

/// A futures-based construct that polls the engine's processes.
#[pin_project(project=EngineProcessesSignalProj)]
pub struct SignalProcessor<E: Backend> {
    #[pin]
    engine: Arc<Mutex<E>>,
}

impl<E: Backend> SignalProcessor<E> {
    pub fn new(engine: Arc<Mutex<E>>) -> Self {
        SignalProcessor { engine }
    }
}

impl<E: Backend> Signal for SignalProcessor<E> {
    type Item = ();

    fn poll_change(self: std::pin::Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let EngineProcessesSignalProj { engine } = self.project();

        let mut engine = engine.lock().expect("Failed to lock ui for polling");
        let engine = engine.deref_mut();
        let engine = unsafe { Pin::new_unchecked(engine) };
        let poll = engine.poll_processors(cx);

        poll
    }
}
