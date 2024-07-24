use super::super::standard_pipeline::StandardUniform;
use crate::app::engine::RenderTarget;
use crate::interaction::{InteractorNode, VecNode};
use crate::reaction::Reactor;
use crate::render_module::RenderModule;
use crate::signals::ReactorProcessor;
use crate::standard::primitive::Primitive;
use futures::stream::select_all;
use futures_signals::signal::{Signal, SignalExt};
use std::pin::Pin;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use wgpu::rwh::RawDisplayHandle;
use wgpu::RenderPipeline;
use winit::dpi::PhysicalSize;

pub struct TupleRenderModule<'window> {
    pub reactors: Vec<Reactor>,
    pub interactors: Vec<Box<dyn InteractorNode>>,
    pub primitive_count: u32,
    pub primitive_buffer_cpu: Vec<Primitive>,
    pub primitive_buffer: wgpu::Buffer,
    pub sub_modules: Vec<Box<dyn RenderModule>>,
    // Currently this is owned, but it should be shared to avoid context-switching.
    pub render_pipeline: RenderModulePipeline<'window>,
}

pub struct RenderModulePipeline<'window> {
    /// This needs to be moved to `RenderState`, i.e., a pipeline doesn't know its own ID.
    pub id: u8,
    pub render_texture: RenderTarget<'window>,
    pub pipeline: RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub uniforms: StandardUniform,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub index_count: u32,
    pub index_buffer: wgpu::Buffer,
}

impl<'window> RenderModulePipeline<'window> {
    pub fn flush_uniforms(&self, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }
}

impl<'window> TupleRenderModule<'window> {
    pub fn new(
        reactors: Vec<Reactor>,
        sub_modules: Vec<Box<dyn RenderModule>>,
        primitive_count: u32,
        primitive_buffer_cpu: Vec<Primitive>,
        primitive_buffer: wgpu::Buffer,
        render_pipeline: RenderModulePipeline<'window>,
    ) -> Self {
        let tuple_render_module = Self {
            reactors,
            interactors: vec![],
            primitive_count,
            primitive_buffer_cpu,
            primitive_buffer,
            sub_modules,
            render_pipeline,
        };

        tuple_render_module
    }

    /// TODO: Write data partially instead of the whole damn buffer.
    pub fn flush_instances(&self, queue: &wgpu::Queue) {
        let slice = bytemuck::cast_slice(&self.primitive_buffer_cpu[..]);
        queue.write_buffer(&self.primitive_buffer, 0, slice);
        queue.submit([]);
    }
}

impl<'window> RenderModule for TupleRenderModule<'window> {
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

    fn resize(
        &mut self,
        new_size: PhysicalSize<u32>,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        adapter: &wgpu::Adapter,
    ) {
        self.render_pipeline
            .render_texture
            .resize(&device, &adapter, new_size);
        self.render_pipeline.uniforms.resize(new_size);
        self.render_pipeline.flush_uniforms(queue);
    }

    fn handle_event(&mut self, event: winit::event::WindowEvent) -> bool {
        let any_handled = false;
        for interactor in self.interactors().iter_mut() {
            interactor.handle_event(event.clone());
        }
        return any_handled;
    }

    fn get_command_encoder(&self, device: &wgpu::Device) -> wgpu::CommandEncoder {
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
    }

    fn draw<'pass>(
        &'pass mut self,
        current_pipeline_id: &mut Option<u8>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'pass>,
    ) {
        self.flush_instances(queue);

        for module in self.sub_modules.iter_mut() {
            module.draw(current_pipeline_id, device, queue, render_pass);
        }

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

    fn present(&self, queue: &wgpu::Queue, command_encoder: wgpu::CommandEncoder) {
        queue.submit(Some(command_encoder.finish()));
    }

    fn reactors(&mut self) -> &mut Vec<Reactor> {
        &mut self.reactors
    }

    fn primitive_buffer(&mut self) -> &mut Vec<Primitive> {
        &mut self.primitive_buffer_cpu
    }

    fn interactors(&mut self) -> &mut Vec<Box<dyn InteractorNode>> {
        &mut self.interactors
    }
}
