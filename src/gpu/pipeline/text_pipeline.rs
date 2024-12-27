use super::GPURenderPipeline;
use crate::gpu::backend::GPUResources;
use crate::gpu::render_target::GPURenderTarget;
use crate::gpu::world::UINodeRenderBuffers;
use crate::prelude::UIItem;
use glyphon::{Cache, FontSystem, SwashCache, TextRenderer};
use vek::{Extent2, Rect};
use wgpu::{ColorTargetState, MultisampleState, Texture, TextureFormat};

/// The pipeline for rendering text.
pub struct TextRenderPipeline {
    pipeline: wgpu::RenderPipeline,
}

impl GPURenderPipeline for TextRenderPipeline {
    fn install_on_render_pass<'pass>(&'pass self, render_pass: &mut wgpu::RenderPass<'pass>) {
        render_pass.set_pipeline(&self.pipeline);
    }

    fn draw(
        gpu_resources: &GPUResources,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        ui_tree: &mut dyn UIItem,
        render_buffers: &mut UINodeRenderBuffers,
    ) {
    }
}

impl TextRenderPipeline {
    pub fn singleton<'a, Target>(
        adapter: &'a wgpu::Adapter,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        render_target_formats: &'a [Option<ColorTargetState>],
    ) where
        Target: GPURenderTarget,
    {
        let mut font_system = FontSystem::new();
        font_system.db_mut().load_font_file("./TestFont.ttf");
        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let viewport = glyphon::Viewport::new(device, &cache);
        let mut atlas =
            glyphon::TextAtlas::new(device, queue, &cache, TextureFormat::Bgra8UnormSrgb);
        let text_renderer =
            TextRenderer::new(&mut atlas, device, MultisampleState::default(), None);
    }
}

/// A single rectangle of text that can be rendered to the screen.
pub struct TextArea {
    buffer: glyphon::Buffer,
    rect: Rect<f32, f32>,
}
