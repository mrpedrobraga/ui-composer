use wgpu::RenderPass;

use super::engine::{LiveNode, UIEngine};

pub mod main_pipeline;

#[cfg(feature = "text")]
pub mod text_pipeline;

/// A render pipeline for rendering on the GPU.
pub trait GPURenderPipeline {
    fn apply_onto<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>);
}
