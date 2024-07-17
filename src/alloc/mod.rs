use crate::app::render_state::RenderTarget;
use std::rc::Rc;
use wgpu::{BufferUsages, RenderPipeline};

pub struct RenderStack<'window> {
    pub reactors: Vec<()>,
    pub primitive_buffer: Vec<u8>,
    pub buffer: wgpu::Buffer,
    pub render_pipeline: Rc<RenderStackPipeline<'window>>,
}

pub struct RenderStackPipeline<'window> {
    pub render_texture: RenderTarget<'window>,
    pub pipeline: RenderPipeline,
    pub device: wgpu::Device,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl<'window> RenderStack<'window> {
    //fn new() {}
}

pub trait RenderModule {
    fn create_render_frame(&self) -> (wgpu::SurfaceTexture, wgpu::TextureView);
    fn prepare<'pass>(&'pass self, render_pass: &mut wgpu::RenderPass<'pass>);
    fn draw(&self, render_pass: &mut wgpu::RenderPass);
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

    fn prepare<'pass>(&'pass self, render_pass: &mut wgpu::RenderPass<'pass>) {
        render_pass.set_pipeline(&self.render_pipeline.pipeline);
    }

    fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.draw(0..3, 0..1);
    }
}

pub trait UIFragment: RenderPipelineProvider {
    fn get_allocation_info(&self) -> AllocationInfo;
    fn push_allocation(&self, render_stack: &mut RenderStack);
}

pub struct AllocationInfo {
    pub buffer_size: u64,
}

pub trait RenderPipelineProvider {
    fn get_render_stack_pipeline<'window>(
        window: std::sync::Arc<winit::window::Window>,
        surface: wgpu::Surface<'window>,
        adapter: &wgpu::Adapter,
        device: wgpu::Device,
    ) -> Rc<RenderStackPipeline<'window>>;
}

pub trait IntoRenderStack {
    fn into_render_stack<'window>(
        self,
        window: std::sync::Arc<winit::window::Window>,
        surface: wgpu::Surface<'window>,
        adapter: &wgpu::Adapter,
        device: wgpu::Device,
    ) -> RenderStack<'window>;
}

impl<TFragment: UIFragment + 'static> IntoRenderStack for TFragment {
    fn into_render_stack<'window>(
        self,
        window: std::sync::Arc<winit::window::Window>,
        surface: wgpu::Surface<'window>,
        adapter: &wgpu::Adapter,
        device: wgpu::Device,
    ) -> RenderStack<'window> {
        let allocation_info = self.get_allocation_info();

        let buffer = (&device).create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: allocation_info.buffer_size,
            usage: BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut render_stack = RenderStack {
            reactors: vec![],
            primitive_buffer: vec![],
            render_pipeline: Self::get_render_stack_pipeline(window, surface, adapter, device),
            buffer,
        };
        self.push_allocation(&mut render_stack);
        return render_stack;
    }
}
