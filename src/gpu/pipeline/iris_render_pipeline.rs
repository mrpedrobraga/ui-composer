//! Pipelines and stuff to render three-dee models!
//! Nothing is implemented yet, of course.

use super::GPURenderPipeline;
use crate::gpu::backend::GPUResources;
use crate::gpu::world::UINodeRenderBuffers;
use crate::prelude::UIItem;
use vek::Extent2;
use wgpu::Texture;

/// The pipeline (and resources) for drawings models in a three dee world.
pub struct IrisRenderPipeline {
    pipeline: wgpu::RenderPipeline,
    pub uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl GPURenderPipeline for IrisRenderPipeline {
    fn install_on_render_pass<'pass>(&'pass self, render_pass: &mut wgpu::RenderPass<'pass>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
    }

    fn draw(
        gpu_resources: &GPUResources,
        render_target_size: Extent2<f32>,
        texture: &Texture,
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
