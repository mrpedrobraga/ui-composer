use super::backend::{GPUResources, Pipelines, RNode};
use crate::gpu::world::UINodeRenderBuffers;
use crate::prelude::UIItem;
use vek::Extent2;
use wgpu::{RenderPass, Texture};

pub mod iris_render_pipeline;
pub mod orchestra_render_pipeline;

#[cfg(feature = "text")]
pub mod text_pipeline;

/// A render pipeline for rendering on the GPU.
pub trait GPURenderPipeline {
    fn draw(
        gpu_resources: &mut GPUResources,
        pipelines: &mut Pipelines,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        ui_tree: &mut dyn UIItem,
        render_buffers: &mut UINodeRenderBuffers,
    );
}
