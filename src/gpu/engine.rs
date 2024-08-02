use futures::{FutureExt, StreamExt};
use futures_signals::signal::{Mutable, SignalExt};
use pollster::FutureExt as _;
use std::sync::{Arc, Mutex, RwLock};
use vek::Extent2;
use winit::{dpi::PhysicalSize, window::Window};

const DEFAULT_CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.02,
    g: 0.02,
    b: 0.02,
    a: 1.0,
};

pub struct GPUResources {
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    adapter: wgpu::Adapter,
}

/// An engine that can render our application to the GPU as well as forward interactive events.
pub struct UIEngine {
    pub gpu_resources: GPUResources,
    pub window_state: WindowState,
    pub window: Arc<Window>,
}

impl UIEngine {
    pub fn new<I>(
        window: Arc<Window>,
        window_state: WindowState,
        root_ui_node: I,
    ) -> Arc<Mutex<Self>> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
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

        let mut render_engine = Self {
            gpu_resources,
            window_state,
            window,
        };

        // This will be shared between the creator of the engine (i.e., your application) and
        // the reactor processor.
        let render_engine = Arc::new(Mutex::new(render_engine));

        // TODO: Process the render engine on another execution context.
        // Perhaps return a `Future` here to be polled by the executor!

        return render_engine;
    }

    pub fn handle_window_event(&mut self, event: winit::event::WindowEvent) {
        unimplemented!()
    }

    pub fn handle_resize(&mut self, new_size: PhysicalSize<u32>) {
        self.window_state
            .window_size
            .set(Extent2::new(new_size.width as f32, new_size.height as f32));

        unimplemented!()
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw()
    }

    pub fn handle_redraw_requested(&mut self) {
        let mut encoder =
            self.gpu_resources
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command Encoder"),
                });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: todo!(),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(DEFAULT_CLEAR_COLOR),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        {
            // TODO: Draw stuff onto render pass!
        }
        drop(render_pass);

        // TODO: Present the frame, in the case of a window redraw!
    }
}

/// The states of a window.
/// TODO: Move out of this file.
pub struct WindowState {
    pub window_size: Mutable<Extent2<f32>>,
}

impl WindowState {
    fn new(window_size: Extent2<f32>) -> Self {
        Self {
            window_size: Mutable::new(window_size),
        }
    }
}
