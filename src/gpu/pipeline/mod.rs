use wgpu::RenderPass;

use super::backend::{Node, WinitWGPUBackend};

pub mod iris_render_pipeline;
pub mod orchestra_render_pipeline;

#[cfg(feature = "text")]
pub mod text_pipeline;

/// A render pipeline for rendering on the GPU.
pub trait GPURenderPipeline {
    fn install_on_render_pass<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>);
}
