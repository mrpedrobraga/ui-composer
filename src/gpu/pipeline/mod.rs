pub mod main_pipeline;

#[cfg(feature = "text")]
pub mod text_pipeline;

/// A render pipeline for rendering on the GPU.
pub trait GPURenderPipeline {}
