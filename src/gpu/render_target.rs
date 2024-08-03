use super::engine::GPUResources;
use std::sync::Arc;
use vek::Extent2;
use wgpu::{
    rwh::{HasDisplayHandle, HasWindowHandle},
    Surface, SurfaceConfiguration, SurfaceTarget, Texture, TextureFormat, TextureView,
};
use winit::{dpi::PhysicalSize, window::Window};

/// Describes a RenderTarget that something can render to.
pub trait GPURenderTarget {
    /// Resizes the render target to the new size.
    fn resize(&mut self, gpu_resources: &GPUResources, new_size: Extent2<u32>);

    /// Returns a reference to the render target's texture;
    fn get_texture_view(&self) -> TextureView;

    /// Returns the texture format;
    fn get_texture_format(&self) -> TextureFormat;
}

/// A render target that will be rendered to a window.
pub struct WindowRenderTarget {
    pub size: Extent2<u32>,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
}

impl WindowRenderTarget {
    pub fn new(gpu_resources: GPUResources, target: Arc<Window>, size: Extent2<u32>) -> Self {
        let surface = gpu_resources.instance.create_surface(target).unwrap();
        let surface_config = surface
            .get_default_config(&gpu_resources.adapter, size.w, size.h)
            .unwrap();

        Self {
            size,
            surface,
            surface_config,
        }
    }

    /// Presents the current texture to the screen.
    pub fn present_texture(&mut self) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to get the current texture of this window to present.");
        surface_texture.present();
    }
}

impl GPURenderTarget for WindowRenderTarget {
    fn resize(&mut self, gpu_resources: &GPUResources, new_size: Extent2<u32>) {
        self.surface_config = self
            .surface
            .get_default_config(&gpu_resources.adapter, new_size.w, new_size.h)
            .unwrap();
        self.surface
            .configure(&gpu_resources.device, &self.surface_config);
    }

    fn get_texture_view(&self) -> TextureView {
        let texture = self
            .surface
            .get_current_texture()
            .expect("Could not get current texture.");

        texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default())
    }

    fn get_texture_format(&self) -> TextureFormat {
        self.surface_config.format
    }
}
