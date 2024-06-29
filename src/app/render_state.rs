use std::sync::Arc;

use pollster::FutureExt as _;
use winit::{dpi::PhysicalSize, window::Window};

pub struct RenderState<'r> {
    pub instance: wgpu::Instance,
    pub main_render_target: RenderTarget<'r>,
    pub device: wgpu::Device,
    pub adapter: wgpu::Adapter,
    pub queue: wgpu::Queue,
    pub main_render_pipeline: wgpu::RenderPipeline,

    pub window: std::sync::Arc<Window>,
}

impl<'r> RenderState<'r> {
    pub fn new(window: Window) -> Self {
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

        let surface_config = surface
            .get_default_config(
                &adapter,
                window.inner_size().width,
                window.inner_size().height,
            )
            .unwrap();
        surface.configure(&device, &surface_config);

        let main_render_target = RenderTarget {
            size: window.inner_size(),
            surface,
            surface_config,
        };

        let shader = device.create_shader_module(wgpu::include_wgsl!("../shader.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let swapchain_capabilities = main_render_target.surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];
        let main_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None, // yet
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            instance,
            main_render_target,
            device,
            adapter,
            queue,
            window,
            main_render_pipeline,
        }
    }

    pub fn handle_resize(&mut self, new_size: PhysicalSize<u32>) {
        self.main_render_target
            .resize(&self.device, &self.adapter, new_size);
        self.request_redraw();
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw()
    }

    pub fn handle_redraw_requested(&mut self) {
        let frame = self
            .main_render_target
            .surface
            .get_current_texture()
            .unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
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

        render_pass.set_pipeline(&self.main_render_pipeline);
        drop(render_pass);

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}

pub struct RenderTarget<'r> {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface<'r>,
    pub surface_config: wgpu::SurfaceConfiguration,
}

impl<'r> RenderTarget<'r> {
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
