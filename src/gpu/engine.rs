use futures::{FutureExt, StreamExt};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use pollster::FutureExt as _;
use std::{
    collections::HashSet,
    marker::PhantomData,
    sync::{Arc, Mutex, RwLock},
    task::{Context, Poll},
};
use vek::Extent2;
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

use crate::ui::node::{LiveUINode, UIEvent, UINode};

use super::{
    pipeline::{
        main_pipeline::{main_render_pipeline, MainRenderPipeline},
        GPURenderPipeline,
    },
    render_target::{self, GPURenderTarget},
    window::{LiveWindowNode, WindowNode, WindowRenderTarget},
};

pub struct GPUResources {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,

    pub main_pipeline: MainRenderPipeline,
}

/// An engine that can render our application to the GPU as well as forward interactive events.
pub struct UIEngine<'engine, E: LiveNode> {
    pub engine_tree: Arc<Mutex<E>>,
    pub gpu_resources: GPUResources,

    _marker: PhantomData<&'engine ()>,
}

pub trait UIEngineInterface: Send {
    type RootNodeType: LiveNode;

    fn handle_window_event(&mut self, window_id: WindowId, event: UIEvent);

    fn poll_reactor_change(&mut self, cx: &mut Context) -> Poll<Option<()>>;
}

impl<'engine: 'static, E: LiveNode + Send + 'engine> UIEngine<'engine, E> {
    pub fn new<W>(
        event_loop: &ActiveEventLoop,
        root_engine_node: W,
    ) -> Arc<Mutex<Box<dyn UIEngineInterface<RootNodeType = E> + 'engine>>>
    where
        W: Node<LiveType = E>,
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
                },
                None,
            )
            .block_on()
            .unwrap();

        // TODO: Not do this:
        let dummy_format = get_dummy_texture_format(event_loop, &instance, &device, &adapter);
        let main_pipeline = main_render_pipeline::<WindowRenderTarget>(
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
            engine_tree: Arc::new(Mutex::new(engine_node)),
            _marker: PhantomData,
            gpu_resources,
        };

        // This will be shared between the creator of the engine (i.e., your application) and
        // the reactor processor.
        let render_engine: Box<dyn UIEngineInterface<RootNodeType = E>> = Box::new(render_engine);
        let render_engine = Arc::new(Mutex::new(render_engine));

        // TODO: Process the render engine's processors and reactors on another execution context.
        // Perhaps return a `Future` here to be polled by the executor!
        let render_engine_clone = render_engine.clone();
        std::thread::spawn(|| {
            pollster::block_on(async move {
                let processor = EngineProcessor::new(render_engine_clone);
            })
        });

        return render_engine;
    }
}

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

impl<'engine, E: LiveNode + Send> UIEngineInterface for UIEngine<'engine, E> {
    type RootNodeType = E;

    fn handle_window_event(&mut self, window_id: WindowId, event: UIEvent) {
        match self.engine_tree.lock() {
            Ok(mut engine_tree) => {
                engine_tree.handle_window_event(&self.gpu_resources, window_id, event)
            }
            Err(_) => unimplemented!("Could not unlock mutex for handling event!"),
        };
    }

    fn poll_reactor_change(&mut self, cx: &mut Context) -> Poll<Option<()>> {
        let mut engine_tree = self
            .engine_tree
            .lock()
            .expect("Couldn't lock engine tree for polling!");

        engine_tree.poll_reactivity_change(cx)
    }
}

pub trait Node: Send {
    type LiveType: LiveNode;
    fn reify(self, event_loop: &ActiveEventLoop, gpu_resources: &GPUResources) -> Self::LiveType;
}

pub trait LiveNode: Send {
    /// Handles an event by broadcasting it around.
    fn handle_window_event(
        &mut self,
        gpu_resources: &GPUResources,
        window_id: WindowId,
        event: UIEvent,
    );

    /// Polls a reactor's change.
    fn poll_reactivity_change(
        &mut self,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>>;
}

/// A futures-based construct that watches and processes the engine for its reactors and processors, etc.
pub struct EngineProcessor<E> {
    engine: Arc<Mutex<Box<dyn UIEngineInterface<RootNodeType = E>>>>,
}

impl<E> EngineProcessor<E> {
    pub fn new(engine: Arc<Mutex<Box<dyn UIEngineInterface<RootNodeType = E>>>>) -> Self {
        EngineProcessor { engine }
    }
}

impl<E: LiveNode> Signal for EngineProcessor<E> {
    type Item = ();

    fn poll_change(self: std::pin::Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let mut any_pending = false;

        let mut ui = self
            .engine
            .lock()
            .expect("Could not lock the engine to process.");

        let poll = ui.poll_reactor_change(cx);

        Poll::Ready(None)
    }
}
