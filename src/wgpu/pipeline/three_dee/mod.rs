#![allow(unused_variables)]
//! Pipelines and stuff to render three-dee models!
//! Nothing is implemented yet, of course.

use crate::wgpu::backend::GPUResources;
use crate::wgpu::render_target::Render;
use {
    super::{GPURenderer, RendererBuffers, Renderers},
    vek::Extent2,
    wgpu::{RenderPass, Texture},
};

/// The pipeline (and resources) for drawings models in a three dee world.
#[allow(unused)]
pub struct ThreeDeeRenderer {
    pipeline: wgpu::RenderPipeline,
    pub uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl GPURenderer for ThreeDeeRenderer {
    fn draw<R: Render>(
        gpu_resources: &mut GPUResources,
        pipelines: &mut Renderers,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        ui_tree: &R,
        render_buffers: &mut RendererBuffers,
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
