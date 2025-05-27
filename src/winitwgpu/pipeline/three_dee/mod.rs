//! Pipelines and stuff to render three-dee models!
//! Nothing is implemented yet, of course.

use {
    super::{GPURenderer, RendererBuffers, Renderers},
    crate::winitwgpu::{backend::Resources, render_target::Render},
    std::marker::PhantomData,
    vek::Extent2,
    wgpu::{RenderPass, Texture},
};

/// The pipeline (and resources) for drawings models in a three dee world.
pub struct IrisRenderer<Tree> {
    pipeline: wgpu::RenderPipeline,
    pub uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    _tree: PhantomData<Tree>,
}

impl<Item> GPURenderer for IrisRenderer<Item> {
    fn draw(
        gpu_resources: &mut Resources,
        pipelines: &mut Renderers,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        ui_tree: &mut dyn Render,
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
