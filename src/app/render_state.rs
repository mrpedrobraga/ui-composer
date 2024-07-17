use crate::{
    alloc::{IntoRenderStack, RenderModule, RenderStack, UIFragment},
    standard::get_main_render_stack_pipeline,
};
use pollster::FutureExt as _;
use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::Window};

pub struct RenderState<'window> {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub queue: wgpu::Queue,
    pub root_render_stack: RenderStack<'window>,

    pub window: std::sync::Arc<Window>,
}

impl<'window> RenderState<'window> {
    pub fn new<TRootFragment: UIFragment + 'static>(
        window: Window,
        root_render_fragment: TRootFragment,
    ) -> Self {
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

        // Allow user to switch the render pipeline!!!
        let root_render_stack = root_render_fragment.into_render_stack(
            get_main_render_stack_pipeline(window.clone(), surface, &adapter, device),
        );

        Self {
            instance,
            adapter,
            queue,
            window,
            root_render_stack,
        }
    }

    pub fn handle_resize(&mut self, new_size: PhysicalSize<u32>) {
        self.root_render_stack.resize(&self.adapter, new_size);
        self.request_redraw();
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw()
    }

    pub fn handle_redraw_requested(&mut self) {
        let (frame, view) = self.root_render_stack.create_render_frame();
        let mut encoder = self
            .root_render_stack
            .render_pipeline
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.05,
                        g: 0.05,
                        b: 0.05,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        {
            self.root_render_stack.prepare(&mut render_pass);
            self.root_render_stack.draw(&mut render_pass);
        }
        drop(render_pass);

        self.queue.submit(Some(encoder.finish()));
        frame.present();
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
