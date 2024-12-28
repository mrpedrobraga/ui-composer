use super::backend::{GPUResources, RNode, Renderers};
use crate::gpu::world::UINodeRenderBuffers;
use crate::prelude::UIItem;
use vek::Extent2;
use wgpu::{RenderPass, Texture};

pub mod iris_renderer;
pub mod orchestra_renderer;

#[cfg(feature = "text")]
pub mod text_rendering;

/// A renderer for drawing on the GPU.
pub trait GPURenderer {
    fn draw(
        gpu_resources: &mut GPUResources,
        renderers: &mut Renderers,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        ui_tree: &mut dyn UIItem,
        render_buffers: &mut UINodeRenderBuffers,
    );
}
