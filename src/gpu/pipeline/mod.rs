use wgpu::RenderPass;

use super::engine::{Node, UIEngine};

pub mod main_pipeline;
pub mod three_dee;

#[cfg(feature = "text")]
pub mod text_pipeline;

/// A render pipeline for rendering on the GPU.
pub trait GPURenderPipeline {
    fn apply_onto<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>);
}
