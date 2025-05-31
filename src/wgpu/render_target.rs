//! A render target is something that graphics, text, etc., can be rendered to.

use crate::app::primitives::PrimitiveDescriptor;
use crate::wgpu::backend::GPUResources;
use crate::wgpu::pipeline::graphics::RenderGraphicDescriptor;
use crate::wgpu::pipeline::{RendererBuffers, Renderers, UIReifyResources};
use {
    super::pipeline::{graphics::RenderGraphic, text::RenderText},
    crate::app::primitives::Primitive,
    vek::Extent2,
    wgpu::TextureFormat,
};

/// Trait that describes the target of a render operation with this pipeline.
pub trait RenderTarget {
    /// Resizes the render target to the new size.
    fn resize(&mut self, gpu_resources: &GPUResources, new_size: Extent2<u32>);

    /// Returns a reference to the render target's texture;
    fn draw<'a, R: Render>(
        &mut self,
        content: &mut R,
        gpu_resources: &mut GPUResources,
        pipelines: &mut Renderers,
        render_buffers: &mut RendererBuffers,
    );

    /// Returns the texture format;
    fn get_texture_format(&self) -> TextureFormat;
}

/// Trait that describes an item that can be rendered with this pipeline.
pub trait Render: Primitive<UIReifyResources> + RenderGraphic + RenderText {}
impl<A> Render for A where A: Primitive<UIReifyResources> + RenderGraphic + RenderText {}

/// Trait that describes an item that can be reified into a [Render] primitive.
pub trait RenderDescriptor: RenderGraphicDescriptor<UIReifyResources, Primitive: Render> {}

impl<A> RenderDescriptor for A
where
    A: PrimitiveDescriptor<UIReifyResources> + RenderGraphicDescriptor<UIReifyResources>,
    <A as PrimitiveDescriptor<UIReifyResources>>::Primitive: Render,
{
}
