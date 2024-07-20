use crate::app::render_state::RenderTarget;
use crate::interaction::InteractorNode;
use crate::render_module::{IntoRenderModule, RenderModule};
use wgpu::util::DeviceExt as _;
use wgpu::{BufferUsages, RenderPipeline};
use winit::dpi::PhysicalSize;

use super::standard_pipeline::{get_main_render_stack_pipeline, StandardUniform};

pub struct RenderStack<'window> {
    pub reactors: Vec<()>,
    pub interactors: Box<dyn InteractorNode>,
    pub primitive_count: u32,
    pub primitive_buffer_cpu: Vec<u8>,
    pub primitive_buffer: wgpu::Buffer,
    pub sub_renderers: (),
    // Currently this is owned, but it should be shared to avoid context-switching.
    pub render_pipeline: RenderStackPipeline<'window>,
}

pub struct RenderStackPipeline<'window> {
    /// This needs to be moved to `RenderState`, i.e., a pipeline doesn't know its own ID.
    pub id: u8,
    pub render_texture: RenderTarget<'window>,
    pub pipeline: RenderPipeline,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
    pub vertex_buffer: wgpu::Buffer,
    pub uniforms: StandardUniform,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub index_count: u32,
    pub index_buffer: wgpu::Buffer,
}

impl<'window> RenderStackPipeline<'window> {
    pub fn send_uniforms(&self) {
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        )
    }
}

impl<'window> RenderModule for RenderStack<'window> {
    fn create_render_frame(&self) -> (wgpu::SurfaceTexture, wgpu::TextureView) {
        let surface_texture = self
            .render_pipeline
            .render_texture
            .surface
            .get_current_texture()
            .unwrap();
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        return (surface_texture, view);
    }

    fn prepare<'pass>(&mut self, _current_pipeline_id: &mut Option<u8>) {}

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.render_pipeline.render_texture.resize(
            &self.render_pipeline.device,
            &self.render_pipeline.adapter,
            new_size,
        );
        self.render_pipeline.uniforms.resize(new_size);
        self.render_pipeline.send_uniforms();
    }

    fn draw<'pass>(&'pass self, render_pass: &mut wgpu::RenderPass<'pass>) {
        render_pass.set_pipeline(&self.render_pipeline.pipeline);
        render_pass.set_bind_group(0, &self.render_pipeline.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.render_pipeline.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.primitive_buffer.slice(..));
        render_pass.set_index_buffer(
            self.render_pipeline.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );

        render_pass.draw_indexed(
            0..self.render_pipeline.index_count,
            0,
            0..self.primitive_count,
        );
    }

    fn get_command_encoder(&self) -> wgpu::CommandEncoder {
        self.render_pipeline
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
    }

    fn present(&self, command_encoder: wgpu::CommandEncoder) {
        self.render_pipeline
            .queue
            .submit(Some(command_encoder.finish()));
    }
}

pub trait UIFragment {
    fn get_allocation_info(&self) -> AllocationInfo;
    fn push_allocation(&self, primitive_buffer: &mut Vec<u8>);
}

pub struct AllocationInfo {
    pub buffer_size: u32,
    pub primitive_count: u32,
}

impl<T> IntoRenderModule for T
where
    T: UIFragment,
{
    fn into_render_module<'window>(
        &self,
        window: std::sync::Arc<winit::window::Window>,
        surface: wgpu::Surface<'window>,
        adapter: wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue,
    ) -> Box<dyn RenderModule + 'window> {
        let render_pipeline =
            get_main_render_stack_pipeline(window.clone(), surface, adapter, device, queue);
        let allocation_info = self.get_allocation_info();

        let mut primitive_buffer = vec![];
        self.push_allocation(&mut primitive_buffer);
        let instance_buffer =
            (&render_pipeline.device).create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&primitive_buffer[..]),
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            });

        let render_stack = RenderStack {
            reactors: vec![],
            interactors: Box::new(crate::interaction::Hover {
                aabb: crate::interaction::Aabb(0.0, 0.0, 100.0, 100.0),
            }),
            sub_renderers: (),
            primitive_count: allocation_info.primitive_count,
            primitive_buffer_cpu: primitive_buffer,
            render_pipeline,
            primitive_buffer: instance_buffer,
        };
        return Box::new(render_stack);
    }
}
