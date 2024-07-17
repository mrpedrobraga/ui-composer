use crate::app::render_state::RenderTarget;
use wgpu::{util::DeviceExt, BufferUsages, RenderPipeline};
use winit::dpi::PhysicalSize;

pub struct InteractorNode {}

pub struct RenderStack<'window> {
    pub reactors: Vec<()>,
    pub interactors: InteractorNode,
    pub primitive_count: u32,
    pub primitive_buffer_cpu: Vec<u8>,
    pub primitive_buffer: wgpu::Buffer,
    pub sub_renderers: (),
    // Currently this is owned, but it should be shared to avoid context-switching.
    pub render_pipeline: RenderModulePipeline<'window>,
}

pub struct RenderModulePipeline<'window> {
    /// This needs to be moved to `RenderState`, i.e., a pipeline doesn't know its own ID.
    pub id: u8,
    pub render_texture: RenderTarget<'window>,
    pub pipeline: RenderPipeline,
    pub device: wgpu::Device,
    pub vertex_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub index_buffer: wgpu::Buffer,
}

impl<'window> RenderStack<'window> {
    //fn new() {}
}

pub trait RenderModule {
    fn create_render_frame(&self) -> (wgpu::SurfaceTexture, wgpu::TextureView);
    fn prepare<'pass>(
        &'pass self,
        current_pipeline_id: &mut Option<u8>,
        render_pass: &mut wgpu::RenderPass<'pass>,
    );
    fn resize(&mut self, adapter: &wgpu::Adapter, new_size: PhysicalSize<u32>);
    fn draw(&self, render_pass: &mut wgpu::RenderPass);
    fn get_pipeline(&self) -> &RenderModulePipeline;
    fn get_command_encoder<'window>(&self) -> wgpu::CommandEncoder {
        self.get_pipeline()
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
    }
}

impl<'window> RenderModule for RenderStack<'window> {
    fn get_pipeline(&self) -> &RenderModulePipeline {
        &self.render_pipeline
    }

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

    fn prepare<'pass>(
        &'pass self,
        current_pipeline_id: &mut Option<u8>,
        render_pass: &mut wgpu::RenderPass<'pass>,
    ) {
        if current_pipeline_id.is_some_and(|id| id == self.render_pipeline.id) {
            return;
        };

        *current_pipeline_id = Some(self.render_pipeline.id);
        render_pass.set_pipeline(&self.render_pipeline.pipeline);
        render_pass.set_vertex_buffer(0, self.render_pipeline.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.primitive_buffer.slice(..));
        render_pass.set_index_buffer(
            self.render_pipeline.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );
    }

    fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.draw_indexed(
            0..self.render_pipeline.index_count,
            0,
            0..self.primitive_count,
        );
    }

    fn resize(&mut self, adapter: &wgpu::Adapter, new_size: PhysicalSize<u32>) {
        self.render_pipeline
            .render_texture
            .resize(&self.render_pipeline.device, adapter, new_size);
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
pub trait IntoRenderModule {
    fn into_render_module<'window>(
        self,
        render_pipeline: RenderModulePipeline<'window>,
    ) -> impl RenderModule;
}

impl<TFragment: UIFragment + 'static> IntoRenderModule for TFragment {
    fn into_render_module<'window>(
        self,
        render_pipeline: RenderModulePipeline<'window>,
    ) -> impl RenderModule {
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
            interactors: InteractorNode {},
            sub_renderers: (),
            primitive_count: allocation_info.primitive_count,
            primitive_buffer_cpu: primitive_buffer,
            render_pipeline,
            primitive_buffer: instance_buffer,
        };
        return render_stack;
    }
}

pub mod ui_fragment_impls;
