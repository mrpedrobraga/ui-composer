use crate::{prelude::AppItemDescriptor, app::node::AppItem};

use super::{backend::GPUResources, pipeline::{graphics::RenderGraphic, text::RenderText}};
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
        content: &mut dyn Render,
        gpu_resources: &mut GPUResources,
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
