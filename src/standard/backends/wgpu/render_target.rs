//! A render target is something that graphics, text, etc., can be rendered to.

use crate::app::composition::reify::Reify;
use crate::standard::backends::wgpu::backend::WgpuResources;
use crate::standard::backends::wgpu::pipeline::graphics::RenderGraphicDescriptor;
use crate::standard::backends::wgpu::pipeline::{RendererBuffers, UIContext, WgpuRenderers};
use {
    super::pipeline::{graphics::RenderGraphic, text::RenderText},
    vek::Extent2,
    wgpu::TextureFormat,
};
use crate::app::composition::algebra::Bubble;
use crate::app::input::Event;
use crate::state::process::Pollable;

/// Trait that describes the target of a render operation with this pipeline.
pub trait RenderTarget {
    /// Resizes the render target to the new size.
    fn resize(&mut self, gpu_resources: &WgpuResources, new_size: Extent2<u32>);

    /// Returns a reference to the render target's texture;
    fn draw<R: RenderBuildingBlock>(
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
pub trait RenderBuildingBlock: Bubble<Event, bool> + Pollable<UIContext> + RenderGraphic + RenderText {}
impl<A> RenderBuildingBlock for A where A: Bubble<Event, bool> + Pollable<UIContext> + RenderGraphic + RenderText {}
/// Trait that describes an item that can be Output into a [RenderBuildingBlock] primitive.
pub trait Render: RenderGraphicDescriptor<UIContext, Output: RenderBuildingBlock> {}

impl<A> Render for A
where
    A: Reify<UIContext> + RenderGraphicDescriptor<UIContext>,
    <A as Reify<UIContext>>::Output: RenderBuildingBlock,
{
}
