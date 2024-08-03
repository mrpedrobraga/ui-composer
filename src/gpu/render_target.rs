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
    fn draw(&mut self, gpu_resources: &GPUResources);

    /// Returns the texture format;
    fn get_texture_format(&self) -> TextureFormat;
}
