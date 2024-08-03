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
    window::{Window, WindowId},
};

use crate::ui::node::{LiveUINode, UIEvent, UINode};

use super::{
    pipeline::{
        main_pipeline::{main_render_pipeline, MainRenderPipeline},
        GPURenderPipeline,
    },
    render_target::{self, GPURenderTarget, WindowRenderTarget},
    window::{LiveWindowNode, WindowNode},
};

const DEFAULT_CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.95,
    g: 0.95,
    b: 0.95,
    a: 1.0,
};

pub struct GPUResources {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
}

/// An engine that can render our application to the GPU as well as forward interactive events.
pub struct UIEngine<'engine, E: LiveNode> {
    pub engine_tree: Arc<Mutex<E>>,
    pub pipelines: HashSet<Box<dyn GPURenderPipeline>>,
    pub main_render_pipeline: MainRenderPipeline,

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

        let gpu_resources = GPUResources {
            instance,
            device,
            queue,
            adapter,
        };

        // TODO: Pass a proper render target!!!
        let main_render_pipeline = main_render_pipeline::<WindowRenderTarget>(
            &gpu_resources.adapter,
            &gpu_resources.device,
            &gpu_resources.queue,
            None,
        );
        let pipelines = HashSet::new();

        let engine_node = root_engine_node.reify(event_loop, gpu_resources);

        let mut render_engine = Self {
            pipelines,
            main_render_pipeline,
            engine_tree: Arc::new(Mutex::new(engine_node)),
            _marker: PhantomData,
        };

        // This will be shared between the creator of the engine (i.e., your application) and
        // the reactor processor.
        let render_engine: Box<dyn UIEngineInterface<RootNodeType = E>> = Box::new(render_engine);
        let render_engine = Arc::new(Mutex::new(render_engine));

        // TODO: Process the render engine's processors and reactors on another execution context.
        // Perhaps return a `Future` here to be polled by the executor!

        return render_engine;
    }

    fn handle_resize(&mut self, new_size: PhysicalSize<u32>) {}

    fn request_redraw(&mut self) {}

    fn handle_redraw_requested(&mut self) {
        // // TODO: This is gonna be a recursive draw,
        // // especially since some UI nodes draw to different render targets.
        // for idx in 0..1 {
        //     let mut encoder =
        //         self.gpu_resources
        //             .device
        //             .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        //                 label: Some("Command Encoder"),
        //             });

        //     /* ------ START OF TODO ------ */
        //     // TODO: Do not create a random ass texture lmao,
        //     // instead switch between render targets from the UI tree.
        //     let surface = self
        //         .gpu_resources
        //         .instance
        //         .create_surface(self.window.clone())
        //         .unwrap();
        //     let surface_caps = surface.get_capabilities(&self.gpu_resources.adapter);
        //     let surface_format = surface_caps
        //         .formats
        //         .iter()
        //         .find(|f| f.is_srgb())
        //         .copied()
        //         .unwrap_or(surface_caps.formats[0]);
        //     let config = wgpu::SurfaceConfiguration {
        //         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //         format: surface_format,
        //         width: self.window_state.window_size.get().w as u32,
        //         height: self.window_state.window_size.get().h as u32,
        //         present_mode: surface_caps.present_modes[0],
        //         alpha_mode: surface_caps.alpha_modes[0],
        //         view_formats: vec![],
        //         desired_maximum_frame_latency: 2,
        //     };
        //     surface.configure(&self.gpu_resources.device, &config);
        //     let render_target_texture = surface
        //         .get_current_texture()
        //         .expect("Failed to get current texture");
        //     let view = render_target_texture
        //         .texture
        //         .create_view(&wgpu::TextureViewDescriptor::default());
        //     /* ------ END OF TODO ------ */

        //     let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        //         label: None,
        //         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        //             view: &view,
        //             resolve_target: None,
        //             ops: wgpu::Operations {
        //                 load: wgpu::LoadOp::Clear(DEFAULT_CLEAR_COLOR),
        //                 store: wgpu::StoreOp::Store,
        //             },
        //         })],
        //         depth_stencil_attachment: None,
        //         timestamp_writes: None,
        //         occlusion_query_set: None,
        //     });

        //     {
        //         // TODO: Render the things that go inside this render target.
        //     }

        //     drop(render_pass);

        //     self.gpu_resources
        //         .queue
        //         .submit(std::iter::once(encoder.finish()));
        //     render_target_texture.present();
        //     drop(surface);
        // }
    }
}

impl<'engine, E: LiveNode> UIEngineInterface for UIEngine<'engine, E> {
    type RootNodeType = E;

    fn handle_window_event(&mut self, window_id: WindowId, event: UIEvent) {
        match self.engine_tree.lock() {
            Ok(mut engine_tree) => engine_tree.handle_window_event(window_id, event),
            Err(_) => unimplemented!("Could not unlock mutex for handling event!"),
        };
    }
}

pub trait Node {
    type LiveType: LiveNode;
    fn reify(self, event_loop: &ActiveEventLoop, gpu_resources: GPUResources) -> Self::LiveType;
}

pub trait LiveNode {
    /// Handles an event by broadcasting it around.
    fn handle_window_event(&mut self, window_id: WindowId, event: UIEvent);
}
