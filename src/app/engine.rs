use crate::render_module::{
    reactivity_processing::{ReactorProcessor, ReactorProcessorEvent},
    IntoRenderModule, RenderModule,
};
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

pub struct UIEngine {
    pub current_pipeline_id: Option<u8>,
    pub root_render_module: Option<Arc<Mutex<Box<dyn RenderModule>>>>,

    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    adapter: wgpu::Adapter,

    pub window_size_state: Mutable<Extent2<f32>>,
    pub window: std::sync::Arc<Window>,
}

impl UIEngine {
    pub fn new<I>(
        window: Window,
        window_size_state: Mutable<Extent2<f32>>,
        root_render_fragment: I,
    ) -> Arc<Mutex<Self>>
    where
        I: IntoRenderModule,
    {
        let instance = wgpu::Instance::default();
        let window = std::sync::Arc::new(window);
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

        let mut render_engine = Self {
            current_pipeline_id: None,
            root_render_module: None,
            window,
            window_size_state,
            instance,
            device,
            queue,
            adapter,
        };

        // TODO: Allow user to switch the render pipeline!!!
        let root_render_module = root_render_fragment.into_render_module(
            render_engine.window.clone(),
            surface,
            &render_engine.adapter,
            &render_engine.device,
            &render_engine.queue,
        );

        // This will be shared between the render engine and the reactor processor.
        let root_render_module = Arc::new(Mutex::new(root_render_module));
        render_engine.root_render_module = Some(root_render_module.clone());

        // This will be shared between the creator of the engine (i.e., your application) and
        // the reactor processor.
        let render_engine = Arc::new(Mutex::new(render_engine));

        let processor = ReactorProcessor::new(root_render_module);
        let render_engine_view = render_engine.clone();
        std::thread::spawn(move || {
            pollster::block_on(processor.for_each(|event| {
                match &event {
                    &ReactorProcessorEvent::DoNothing => (),
                    &ReactorProcessorEvent::Redraw => {
                        render_engine_view
                            .lock()
                            .expect("Could not lock Render Engine to request redraw!")
                            .request_redraw();
                    }
                }
                async {}
            }))
        });

        return render_engine;
    }

    pub fn handle_window_event(&mut self, event: winit::event::WindowEvent) {
        if let Some(root_render_module) = &mut self.root_render_module {
            let mut root_render_module = root_render_module
                .lock()
                .expect("Couldn't lock Root Render Module for window event.");
            root_render_module.handle_event(event);
        }
    }

    pub fn handle_resize(&mut self, new_size: PhysicalSize<u32>) {
        self.window_size_state
            .set(Extent2::new(new_size.width as f32, new_size.height as f32));

        if let Some(root_render_module) = &mut self.root_render_module {
            let mut root_render_module = root_render_module
                .lock()
                .expect("Couldn't lock Root Render Module for resize event.");
            root_render_module.resize(new_size, &self.queue, &self.device, &self.adapter);
            drop(root_render_module);
            self.request_redraw();
        }
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw()
    }

    pub fn handle_redraw_requested(&mut self) {
        if let Some(root_render_module) = &mut self.root_render_module {
            let mut root_render_module = root_render_module
                .lock()
                .expect("Couldn't lock Root Render Module for redraw requested event.");
            let (frame, view) = root_render_module.create_render_frame();
            let mut encoder = root_render_module.get_command_encoder(&self.device);

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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
                self.current_pipeline_id = None;
                root_render_module.draw(
                    &mut self.current_pipeline_id,
                    &self.device,
                    &self.queue,
                    &mut render_pass,
                );
            }
            drop(render_pass);

            root_render_module.present(&self.queue, encoder);
            frame.present();
        }
    }
}

/// Describes a RenderTarget that a render module can render to.
/// TODO: Move this out of here.
pub struct RenderTarget<'window> {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface<'window>,
    pub surface_config: wgpu::SurfaceConfiguration,
}

impl<'window> RenderTarget<'window> {
    pub fn new(
        instance: &wgpu::Instance,
        adapter: &wgpu::Adapter,
        target: Arc<winit::window::Window>,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Self {
        let surface = instance.create_surface(target).unwrap();
        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        Self {
            size,
            surface,
            surface_config,
        }
    }

    pub fn resize(
        &mut self,
        device: &wgpu::Device,
        adapter: &wgpu::Adapter,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) {
        let surface_config = self
            .surface
            .get_default_config(&adapter, new_size.width, new_size.height)
            .unwrap();
        self.surface.configure(&device, &surface_config);
    }
}
