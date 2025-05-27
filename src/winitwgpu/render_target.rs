//! A render target is something that graphics, text, etc, can be rendereed to.

use {
    super::{
        backend::Resources,
        pipeline::{graphics::RenderGraphic, text::RenderText},
    },
    crate::{
        app::node::{AppItem, AppItemDescriptor},
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
        content: &mut dyn Render,
        gpu_resources: &mut Resources,
        pipelines: &mut Renderers,
        render_buffers: &mut RendererBuffers,
    );

    /// Returns the texture format;
    fn get_texture_format(&self) -> TextureFormat;
}

pub trait Render: AppItem + RenderGraphic + RenderText {}
impl<A> Render for A where A: AppItem + RenderGraphic + RenderText {}

pub trait RenderDescriptor: Render + AppItemDescriptor {}
impl<A> RenderDescriptor for A where A: Render + AppItemDescriptor {}
