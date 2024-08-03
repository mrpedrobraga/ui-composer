use futures::{FutureExt, StreamExt};
use futures_signals::signal::{Mutable, SignalExt};
use pollster::FutureExt as _;
use std::{
    collections::HashSet,
    marker::PhantomData,
    sync::{Arc, Mutex, RwLock},
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

pub trait UIEngineInterface {
    type RootNodeType: LiveNode;

    fn handle_window_event(&mut self, window_id: WindowId, event: UIEvent);
}

impl<'engine, E: LiveNode + 'engine> UIEngine<'engine, E> {
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

        let dummy_window = event_loop
            .create_window(WindowAttributes::default())
            .expect("Failure to create dummy window");
        let dummy_surface = instance
            .create_surface(dummy_window)
            .expect("Failure to get dummy window surface.");
        dummy_surface.configure(
            &device,
            &dummy_surface
                .get_default_config(&adapter, 20, 20)
                .expect("No valid config between this surface and the adapter."),
        );
        let dummy_texture = dummy_surface
            .get_current_texture()
            .expect("Failure to get dummy texture.");
        let dummy_format = dummy_texture.texture.format();

        // TODO: Pass a proper render target!!!
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

        return render_engine;
    }
}

impl<'engine, E: LiveNode> UIEngineInterface for UIEngine<'engine, E> {
    type RootNodeType = E;

    fn handle_window_event(&mut self, window_id: WindowId, event: UIEvent) {
        match self.engine_tree.lock() {
            Ok(mut engine_tree) => {
                engine_tree.handle_window_event(&self.gpu_resources, window_id, event)
            }
            Err(_) => unimplemented!("Could not unlock mutex for handling event!"),
        };
    }
}

pub trait Node {
    type LiveType: LiveNode;
    fn reify(self, event_loop: &ActiveEventLoop, gpu_resources: &GPUResources) -> Self::LiveType;
}

pub trait LiveNode {
    /// Handles an event by broadcasting it around.
    fn handle_window_event(
        &mut self,
        gpu_resources: &GPUResources,
        window_id: WindowId,
        event: UIEvent,
    );
}
