use winit::dpi::PhysicalSize;

pub trait RenderModule {
    fn create_render_frame(&self) -> (wgpu::SurfaceTexture, wgpu::TextureView);
    fn prepare<'pass>(
        &'pass mut self,
        current_pipeline_id: &mut Option<u8>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        // render_pass: &mut wgpu::RenderPass<'pass>,
    );
    fn resize(
        &mut self,
        new_size: PhysicalSize<u32>,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        adapter: &wgpu::Adapter,
    );
    fn handle_event(&mut self, event: winit::event::WindowEvent) -> bool;
    fn get_command_encoder(&self, device: &wgpu::Device) -> wgpu::CommandEncoder;
    fn draw<'pass>(&'pass self, render_pass: &mut wgpu::RenderPass<'pass>);
    fn present(&self, queue: &wgpu::Queue, encoder: wgpu::CommandEncoder);
}

pub trait IntoRenderModule {
    fn into_render_module<'a, 'window>(
        self,
        window: std::sync::Arc<winit::window::Window>,
        surface: wgpu::Surface<'window>,
        adapter: &'a wgpu::Adapter,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
    ) -> Box<dyn RenderModule + 'window>;
}
