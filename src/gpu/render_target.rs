use crate::ui::node::UIItem;

use super::{backend::GPUResources, world::UINodeRenderBuffers};
use crate::gpu::backend::Renderers;
use vek::Extent2;
use wgpu::TextureFormat;

/// Describes a RenderTarget that something can render to.
pub trait GPURenderTarget {
    /// Resizes the render target to the new size.
    fn resize(&mut self, gpu_resources: &GPUResources, new_size: Extent2<u32>);

    /// Returns a reference to the render target's texture;
    fn draw(
        &mut self,
        gpu_resources: &mut GPUResources,
        pipelines: &mut Renderers,
        content: &mut dyn UIItem,
        render_buffers: &mut UINodeRenderBuffers,
    );

    /// Returns the texture format;
    fn get_texture_format(&self) -> TextureFormat;
}
