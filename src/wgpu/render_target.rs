//! A render target is something that graphics, text, etc., can be rendered to.

use crate::app::building_blocks::Reifiable;
use crate::wgpu::backend::WgpuResources;
use crate::wgpu::pipeline::graphics::RenderGraphicDescriptor;
use crate::wgpu::pipeline::{RendererBuffers, WgpuRenderers, UIContext};
use {
    super::pipeline::{graphics::RenderGraphic, text::RenderText},
    crate::app::building_blocks::BuildingBlock,
    vek::Extent2,
    wgpu::TextureFormat,
};

/// Trait that describes the target of a render operation with this pipeline.
pub trait RenderTarget {
    /// Resizes the render target to the new size.
    fn resize(&mut self, gpu_resources: &WgpuResources, new_size: Extent2<u32>);

    /// Returns a reference to the render target's texture;
    fn draw<R: RenderWgpu>(
        &mut self,
        content: &mut R,
        gpu_resources: &mut WgpuResources,
        pipelines: &mut WgpuRenderers,
        render_buffers: &mut RendererBuffers,
    );

    /// Returns the texture format;
    fn get_texture_format(&self) -> TextureFormat;
}

/// Trait that describes an item that can be rendered with this pipeline.
pub trait RenderWgpu: BuildingBlock<UIContext> + RenderGraphic + RenderText {}
impl<A> RenderWgpu for A where A: BuildingBlock<UIContext> + RenderGraphic + RenderText {}
/// Trait that describes an item that can be reified into a [RenderWgpu] primitive.
pub trait RenderDescriptor: RenderGraphicDescriptor<UIContext, Reified: RenderWgpu> {}

impl<A> RenderDescriptor for A
where
    A: Reifiable<UIContext> + RenderGraphicDescriptor<UIContext>,
    <A as Reifiable<UIContext>>::Reified: RenderWgpu,
{
}
