use crate::wgpu::backend::GPUResources;
use crate::wgpu::pipeline::{
    graphics::{GraphicsPipelineBuffers, OrchestraRenderer},
    text::GlyphonTextRenderer,
};
use crate::wgpu::render_target::Render;
use {
    text::TextPipelineBuffers,
    vek::Extent2,
    wgpu::{RenderPass, Texture},
};

pub mod graphics;
pub mod text;
pub mod three_dee;

/// A renderer for drawing on the GPU.
pub trait GPURenderer {
    fn draw<'draw, R: Render>(
        gpu_resources: &mut GPUResources,
        renderers: &mut Renderers,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        // TODO: Maybe each [GPURenderer] should be able to specify different bounds for the UI tree...
        ui_tree: &'draw R,
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
    pub(crate) _text_render_buffers: TextPipelineBuffers,
}

pub struct UIReifyResources {
    pub renderers: Renderers,
}
