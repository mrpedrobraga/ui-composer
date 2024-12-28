use super::backend::{GPUResources, RNode};
use crate::gpu::pipeline::orchestra_renderer::{GraphicsPipelineBuffers, OrchestraRenderer};
use crate::gpu::pipeline::text_rendering::GlyphonTextRenderer;
use crate::prelude::UIItem;
use vek::Extent2;
use wgpu::{RenderPass, Texture};

pub mod iris_renderer;
pub mod orchestra_renderer;

#[cfg(feature = "text")]
pub mod text_rendering;

/// A renderer for drawing on the GPU.
pub trait GPURenderer {
    fn draw(
        gpu_resources: &mut GPUResources,
        renderers: &mut Renderers,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        ui_tree: &mut dyn UIItem,
        render_buffers: &mut RendererBuffers,
    );
}

/// The struct containing all the different renderers that
/// composite the scene together.
/// TODO: Make this variadic, generic.
pub struct Renderers {
    pub graphics_renderer: OrchestraRenderer,
    pub text_renderer: GlyphonTextRenderer,
}

pub struct RendererBuffers {
    pub(crate) graphics_render_buffers: GraphicsPipelineBuffers,
}
