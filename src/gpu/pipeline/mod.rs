use wgpu::RenderPass;

use super::engine::{Node, UIEngine};

pub mod orchestra_render_pipeline;
pub mod three_dee;

#[cfg(feature = "text")]
pub mod text_pipeline;

/// A render pipeline for rendering on the GPU.
pub trait GPURenderPipeline {
    fn install_on_render_pass<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>);
}
