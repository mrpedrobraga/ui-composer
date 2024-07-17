use crate::{
    alloc::{self, RenderPipelineProvider, RenderStackPipeline, UIFragment},
    app::render_state::RenderTarget,
};
use bytemuck::{Pod, Zeroable};
use std::mem::size_of;
use wgpu::BufferUsages;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Primitive {
    pub color: [f32; 3],
}

impl UIFragment for Primitive {
    fn get_allocation_info(&self) -> alloc::AllocationInfo {
        alloc::AllocationInfo {
            buffer_size: size_of::<Self>() as u64,
        }
    }

    fn push_allocation(&self, render_stack: &mut alloc::RenderStack) {
        render_stack
            .primitive_buffer
            .extend(bytemuck::cast_slice(&[*self]))
    }
}

impl RenderPipelineProvider for Primitive {
    fn get_render_stack_pipeline<'window>(
        window: std::sync::Arc<winit::window::Window>,
        surface: wgpu::Surface<'window>,
        adapter: &wgpu::Adapter,
        device: wgpu::Device,
    ) -> std::rc::Rc<alloc::RenderStackPipeline<'window>> {
        let surface_config = surface
            .get_default_config(
                &adapter,
                window.inner_size().width,
                window.inner_size().height,
            )
            .unwrap();
        surface.configure(&device, &surface_config);

        let render_target = RenderTarget {
            size: window.inner_size(),
            surface,
            surface_config,
        };

        let swapchain_capabilities = render_target.surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let shader = device.create_shader_module(wgpu::include_wgsl!("./standard-shader.wgsl"));
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 0,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 0,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        std::rc::Rc::new(RenderStackPipeline {
            pipeline: render_pipeline,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            render_texture: render_target,
            device,
        })
    }
}
