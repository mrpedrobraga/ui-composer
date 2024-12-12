use crate::ui::node::UINode;

use super::{engine::GPUResources, world::UINodeRenderBuffers};
use std::sync::Arc;
use vek::Extent2;
use wgpu::{
    rwh::{HasDisplayHandle, HasWindowHandle},
    RenderPass, Surface, SurfaceConfiguration, SurfaceTarget, Texture, TextureFormat, TextureView,
};
use winit::{dpi::PhysicalSize, window::Window};

/// Describes a RenderTarget that something can render to.
pub trait GPURenderTarget {
    /// Resizes the render target to the new size.
    fn resize(&mut self, gpu_resources: &GPUResources, new_size: Extent2<u32>);

    /// Returns a reference to the render target's texture;
    fn draw(
        &mut self,
        gpu_resources: &GPUResources,
        content: &dyn UINode,
        render_buffers: &mut UINodeRenderBuffers,
    );

    /// Returns the texture format;
    fn get_texture_format(&self) -> TextureFormat;
}
