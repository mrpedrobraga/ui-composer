use crate::render_module::{IntoRenderModule, RenderModule};
use pollster::FutureExt as _;
use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::Window};

const DEFAULT_CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.02,
    g: 0.02,
    b: 0.02,
    a: 1.0,
};

pub struct UIEngine {
    pub current_pipeline_id: Option<u8>,
    pub root_render_module: Option<Box<dyn RenderModule>>,

    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    adapter: wgpu::Adapter,

    pub window: std::sync::Arc<Window>,
}

impl UIEngine {
    pub fn new<I>(window: Window, root_render_fragment: I) -> Self
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

        let mut render_state = Self {
            current_pipeline_id: None,
            root_render_module: None,
            window,
            instance,
            device,
            queue,
            adapter,
        };

        // Allow user to switch the render pipeline!!!
        let root_render_stack = root_render_fragment.into_render_module(
            render_state.window.clone(),
            surface,
            &render_state.adapter,
            &render_state.device,
            &render_state.queue,
        );

        render_state.root_render_module = Some(root_render_stack);

        render_state
    }

    pub fn handle_window_event(&mut self, event: winit::event::WindowEvent) {
        if let Some(root_render_module) = &mut self.root_render_module {
            root_render_module.handle_event(event);
        }
    }

    pub fn handle_resize(&mut self, new_size: PhysicalSize<u32>) {
        if let Some(root_render_module) = &mut self.root_render_module {
            root_render_module.resize(new_size, &self.queue, &self.device, &self.adapter);
            self.request_redraw();
        }
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw()
    }

    pub fn handle_redraw_requested(&mut self) {
        if let Some(root_render_module) = &mut self.root_render_module {
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
                root_render_module.prepare(
                    &mut self.current_pipeline_id,
                    &self.device,
                    &self.queue,
                    //&mut render_pass,
                );
                root_render_module.draw(&mut render_pass);
            }
            drop(render_pass);

            // Probably keep the queue here in the render state???
            root_render_module.present(&self.queue, encoder);
            frame.present();
        }
    }
}

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
