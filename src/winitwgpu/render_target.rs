//! A render target is something that graphics, text, etc, can be rendereed to.

use {
    super::{
        backend::Resources,
        pipeline::{
            graphics::{RenderGraphic, RenderGraphicDescriptor},
            text::RenderText,
        },
    },
    crate::{
        app::primitives::{Primitive, PrimitiveDescriptor},
        winitwgpu::pipeline::{RendererBuffers, Renderers},
    },
    vek::Extent2,
    wgpu::TextureFormat,
};

pub trait RenderTarget {
    /// Resizes the render target to the new size.
    fn resize(&mut self, gpu_resources: &Resources, new_size: Extent2<u32>);

    /// Returns a reference to the render target's texture;
    fn draw(
        &mut self,
        content: &mut dyn RenderInternal,
        gpu_resources: &mut Resources,
        pipelines: &mut Renderers,
        render_buffers: &mut RendererBuffers,
    );

    /// Returns the texture format;
    fn get_texture_format(&self) -> TextureFormat;
}

pub trait RenderInternal: Primitive + RenderGraphic + RenderText {}
impl<A> RenderInternal for A where A: Primitive + RenderGraphic + RenderText {}

pub trait Render:
    RenderInternal + PrimitiveDescriptor + RenderGraphicDescriptor + RenderText
{
}
impl<A> Render for A where A: RenderInternal + PrimitiveDescriptor + RenderGraphicDescriptor {}
