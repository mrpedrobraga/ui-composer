use winit::dpi::PhysicalSize;

pub trait RenderModule {
    fn create_render_frame(&self) -> (wgpu::SurfaceTexture, wgpu::TextureView);
    fn prepare(&mut self, current_pipeline_id: &mut Option<u8>);
    fn resize(&mut self, new_size: PhysicalSize<u32>);
    fn handle_event(&mut self, event: winit::event::WindowEvent) -> bool;
    fn get_command_encoder(&self) -> wgpu::CommandEncoder;
    fn draw<'pass>(&'pass self, render_pass: &mut wgpu::RenderPass<'pass>);
    fn present(&self, encoder: wgpu::CommandEncoder);
}

pub trait IntoRenderModule {
    fn into_render_module<'window>(
        &self,
        window: std::sync::Arc<winit::window::Window>,
        surface: wgpu::Surface<'window>,
        adapter: wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue,
    ) -> Box<dyn RenderModule + 'window>;
}
