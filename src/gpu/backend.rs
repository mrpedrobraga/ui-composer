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
    type Event;

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

/// An engine that can render our application to the GPU as well as forward interactive events to the app.
#[pin_project(project=UIEngineProj)]
pub struct WinitWGPUBackend<'engine, E: Node> {
    /// The node of the UI tree containing the entirety of the app, UI and behaviour.
    #[pin]
    pub node_tree: Arc<Mutex<E>>,
    pub gpu_resources: GPUResources,
    _marker: PhantomData<&'engine ()>,
}

impl<'engine, E: Node> Backend for WinitWGPUBackend<'engine, E> {
    type Event = winit::event::WindowEvent;

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        let UIEngineProj {
            node_tree,
            gpu_resources,
            _marker,
        } = self.project();

        let mut engine_tree = node_tree.lock().expect("Failed to lock tree for polling");
        let engine_tree = engine_tree.deref_mut();
        let engine_tree_pin = unsafe { Pin::new_unchecked(engine_tree) };

        engine_tree_pin.poll_processors(cx)
    }
}

pub trait WinitBackend: Send {
    type RootNodeType: Node;
    fn handle_resumed(&mut self);
    fn handle_window_event(&mut self, window_id: WindowId, event: UIEvent);
}

impl<'engine: 'static, E: Node + Send + 'engine> WinitWGPUBackend<'engine, E> {
    pub fn new<W>(
        event_loop: &ActiveEventLoop,
        root_engine_node: W,
    ) -> Arc<Mutex<Box<dyn WinitBackend<RootNodeType = E> + 'engine>>>
    where
        W: NodeDescriptor<RuntimeType = E>,
    {
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

        let engine_node = root_engine_node.reify(event_loop, &gpu_resources);

        let mut render_engine = Self {
            node_tree: Arc::new(Mutex::new(engine_node)),
            _marker: PhantomData,
            gpu_resources,
        };

        // This render engine will be shared between your application (so that it can receive OS events)
        // and a reactor processor in another thread (so that it can process its own `Future`s and `Signal`s)
        let render_engine: Box<dyn WinitBackend<RootNodeType = E>> = Box::new(render_engine);
        let render_engine = Arc::new(Mutex::new(render_engine));

        // TODO: Process the render engine's processors and reactors on another thread.
        // It needs not be an other thread, just a different execution context.
        let render_engine_clone = render_engine.clone();
        std::thread::spawn(|| {
            pollster::block_on(EngineProcessesSignal::new(render_engine_clone).to_future())
        });

        // I should perhaps return a (RenderEngine, impl Future) here!
        // The problem of course is that winit does not play well with async Rust yet :-(
        return render_engine;
    }

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

impl<'engine, E: Node + Send> WinitBackend for WinitWGPUBackend<'engine, E> {
    type RootNodeType = E;

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
    type RuntimeType: Node;
    fn reify(self, event_loop: &ActiveEventLoop, gpu_resources: &GPUResources)
        -> Self::RuntimeType;
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
    type RuntimeType = (A::RuntimeType, B::RuntimeType);

    fn reify(
        self,
        event_loop: &ActiveEventLoop,
        gpu_resources: &GPUResources,
    ) -> Self::RuntimeType {
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
pub struct EngineProcessesSignal<E> {
    #[pin]
    engine: Arc<Mutex<Box<dyn Backend<Event = E>>>>,
}

impl<E> EngineProcessesSignal<E> {
    pub fn new(engine: Arc<Mutex<Box<dyn Backend<Event = E>>>>) -> Self {
        EngineProcessesSignal { engine }
    }
}

impl<E> Signal for EngineProcessesSignal<E> {
    type Item = ();

    fn poll_change(self: std::pin::Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let EngineProcessesSignalProj { engine } = self.project();

        let mut engine = engine.lock().expect("Failed to lock ui for polling");
        let engine: &mut dyn Backend<Event = E> = engine.as_mut();
        let engine = unsafe { Pin::new_unchecked(engine) };

        let poll = engine.poll_processors(cx);

        poll
    }
}
