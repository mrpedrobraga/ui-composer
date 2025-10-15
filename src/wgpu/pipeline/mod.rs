use crate::wgpu::backend::WgpuResources;
use crate::wgpu::pipeline::{
    graphics::{GraphicsPipelineBuffers, OrchestraRenderer},
    text::TextRenderer,
};
use crate::wgpu::render_target::RenderBuildingBlock;
use {
    text::TextPipelineResources,
    vek::Extent2,
    wgpu::{RenderPass, Texture},
};

pub mod graphics;
pub mod text;
pub mod three_dee;

/// A renderer for drawing on the GPU.
pub trait WgpuRenderer {
    fn draw<R: RenderBuildingBlock>(
        gpu_resources: &mut WgpuResources,
        renderers: &mut WgpuRenderers,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        // TODO: Maybe each [GPURenderer] should be able to specify
        // different bounds for the UI tree...
        ui_tree: &R,
        render_buffers: &mut RendererBuffers,
    );
}

/// The struct containing all the different renderers that
/// composite the scene together.
/// TODO: Make this variadic, generic.
pub struct WgpuRenderers {
    pub graphics_renderer: OrchestraRenderer,
    pub text_renderer: TextRenderer,
}

pub struct RendererBuffers {
    pub(crate) graphics_render_buffers: GraphicsPipelineBuffers,
    pub(crate) _text_render_buffers: TextPipelineResources,
}

pub struct UIContext {
    pub renderers: WgpuRenderers,
}
