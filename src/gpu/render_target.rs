use crate::ui::node::UIItem;

use super::{backend::GPUResources, pipeline::graphics::GraphicItem};
use crate::gpu::pipeline::{RendererBuffers, Renderers};
use vek::Extent2;
use wgpu::TextureFormat;

/// Describes a RenderTarget that something can render to.
pub trait GPURenderTarget {
    /// Resizes the render target to the new size.
    fn resize(&mut self, gpu_resources: &GPUResources, new_size: Extent2<u32>);

    /// Returns a reference to the render target's texture;
    fn draw(
        &mut self,
        content: &mut dyn RenderTargetContent,
        gpu_resources: &mut GPUResources,
        pipelines: &mut Renderers,
        render_buffers: &mut RendererBuffers,
    );

    /// Returns the texture format;
    fn get_texture_format(&self) -> TextureFormat;
}

pub trait RenderTargetContent: UIItem + GraphicItem {}
impl<A> RenderTargetContent for A where A: UIItem + GraphicItem {}