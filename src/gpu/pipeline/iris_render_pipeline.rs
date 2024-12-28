//! Pipelines and stuff to render three-dee models!
//! Nothing is implemented yet, of course.

use super::GPURenderPipeline;
use crate::gpu::backend::{GPUResources, Pipelines};
use crate::gpu::world::UINodeRenderBuffers;
use crate::prelude::UIItem;
use vek::Extent2;
use wgpu::{RenderPass, Texture};

/// The pipeline (and resources) for drawings models in a three dee world.
pub struct IrisRenderPipeline {
    pipeline: wgpu::RenderPipeline,
    pub uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl GPURenderPipeline for IrisRenderPipeline {
    fn draw(
        gpu_resources: &mut GPUResources,
        pipelines: &mut Pipelines,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        ui_tree: &mut dyn UIItem,
        render_buffers: &mut UINodeRenderBuffers,
    ) {
    }
}

/// The GPU resources of a single model.
/// With this you should be able to draw a single model once or many times.
pub struct ModelRenderBuffers {
    pub mesh_vertex_buffer: wgpu::Buffer,
    pub mesh_index_buffer: wgpu::Buffer,
    pub mesh_index_count: usize,
}

pub struct Material {}
