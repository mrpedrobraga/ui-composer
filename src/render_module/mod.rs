use winit::dpi::PhysicalSize;
pub mod reactivity_processing;
use crate::{interaction::InteractorNode, prelude::Primitive, reaction::Reactor};

pub trait RenderModule: Send {
    fn initial(&self) -> bool;
    fn set_setup_finished(&mut self);
    fn create_render_frame(&self) -> (wgpu::SurfaceTexture, wgpu::TextureView);
    fn resize(
        &mut self,
        new_size: PhysicalSize<u32>,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        adapter: &wgpu::Adapter,
    );
    fn handle_event(&mut self, event: winit::event::WindowEvent) -> bool;
    fn get_command_encoder(&self, device: &wgpu::Device) -> wgpu::CommandEncoder;
    fn draw<'pass>(
        &'pass mut self,
        current_pipeline_id: &mut Option<u8>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'pass>,
    );
    fn present(&self, queue: &wgpu::Queue, encoder: wgpu::CommandEncoder);

    ///TODO: Make the type signature perhaps not use Option?
    fn reactors(&mut self) -> &mut Vec<Option<Reactor>>;
    fn primitive_buffer(&mut self) -> &mut Vec<Primitive>;
    ///TODO: Get rid of the Option? Maybe perhaps even the Box<dyn InteractorNode>?
    fn interactors(&mut self) -> &mut Vec<Option<Box<dyn InteractorNode>>>;
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
