use super::backend::{GPUResources, ReifiedNode};
use super::render_target::Render;
use crate::gpu::pipeline::graphics::{GraphicsPipelineBuffers, OrchestraRenderer};
use crate::gpu::pipeline::text::GlyphonTextRenderer;
use crate::prelude::AppItem;
use text::TextPipelineBuffers;
use vek::Extent2;
use wgpu::{RenderPass, Texture};

pub mod graphics;
pub mod text;
pub mod three_dee;

/// A renderer for drawing on the GPU.
pub trait GPURenderer {
    fn draw(
        gpu_resources: &mut GPUResources,
        renderers: &mut Renderers,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        ui_tree: &mut dyn Render,
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
    pub(crate) text_render_buffers: TextPipelineBuffers,
}
