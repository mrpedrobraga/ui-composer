use {
    super::{backend::Resources, render_target::RenderInternal},
    crate::winitwgpu::pipeline::{
        graphics::{GraphicsPipelineBuffers, OrchestraRenderer},
        text::GlyphonTextRenderer,
    },
    text::TextPipelineBuffers,
    vek::Extent2,
    wgpu::{RenderPass, Texture},
};

pub mod graphics;
pub mod text;
pub mod three_dee;

/// A renderer for drawing on the GPU.
pub trait GPURenderer {
    fn draw(
        gpu_resources: &mut Resources,
        renderers: &mut Renderers,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        ui_tree: &mut dyn RenderInternal,
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
