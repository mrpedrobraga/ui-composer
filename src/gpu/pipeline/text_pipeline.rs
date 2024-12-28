use super::GPURenderPipeline;
use crate::gpu::backend::{GPUResources, Pipelines};
use crate::gpu::render_target::GPURenderTarget;
use crate::gpu::world::UINodeRenderBuffers;
use crate::prelude::UIItem;
use glyphon::{
    Cache, FontSystem, Resolution, SwashCache, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use vek::{Extent2, Rect};
use wgpu::{ColorTargetState, MultisampleState, RenderPass, Texture, TextureFormat};

/// The pipeline for rendering text.
pub struct TextRenderPipeline {
    text_renderer: TextRenderer,
    font_system: FontSystem,
    viewport: Viewport,
    atlas: TextAtlas,
    cache: Cache,
    swash_cache: SwashCache,
}

impl GPURenderPipeline for TextRenderPipeline {
    fn draw(
        gpu_resources: &mut GPUResources,
        pipelines: &mut Pipelines,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        ui_tree: &mut dyn UIItem,
        render_buffers: &mut UINodeRenderBuffers,
    ) {
        let this = &mut pipelines.text_pipeline;

        this.viewport.update(
            &gpu_resources.queue,
            Resolution {
                width: texture.width(),
                height: texture.height(),
            },
        );

        let mut buffer =
            cosmic_text::Buffer::new(&mut this.font_system, cosmic_text::Metrics::new(16.0, 20.0));

        buffer.set_text(
            &mut this.font_system,
            "عيد ميلاد مجيد!",
            cosmic_text::Attrs::new().family(cosmic_text::Family::Name("Work Sans")),
            cosmic_text::Shaping::Advanced,
        );
        buffer.set_size(
            &mut this.font_system,
            Some(texture.width() as f32),
            Some(texture.height() as f32),
        );
        buffer.set_wrap(&mut this.font_system, cosmic_text::Wrap::Word);
        buffer.shape_until_scroll(&mut this.font_system, false);

        let text_areas = [glyphon::TextArea {
            buffer: &buffer,
            left: 0.0,
            top: 0.0,
            scale: 1.0,
            bounds: TextBounds {
                left: 0,
                top: 0,
                right: texture.width() as i32,
                bottom: texture.height() as i32,
            },
            default_color: glyphon::Color::rgb(255, 255, 255),
            custom_glyphs: &[],
        }];

        this.text_renderer
            .prepare(
                &gpu_resources.device,
                &gpu_resources.queue,
                &mut this.font_system,
                &mut this.atlas,
                &this.viewport,
                text_areas,
                &mut this.swash_cache,
            )
            .unwrap();

        this.text_renderer
            .render(&this.atlas, &this.viewport, render_pass)
            .unwrap()
    }
}

impl TextRenderPipeline {
    pub fn singleton<'a, Target>(
        adapter: &'a wgpu::Adapter,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        render_target_formats: &'a [Option<ColorTargetState>],
    ) -> Self
    where
        Target: GPURenderTarget,
    {
        let mut font_system = FontSystem::new();
        font_system
            .db_mut()
            .load_font_file("./src/gpu/pipeline/WorkSans-Regular.ttf")
            .expect("Failed to load font!");
        font_system
            .db_mut()
            .load_font_file("./src/gpu/pipeline/Anima Sans.ttf")
            .expect("Failed to load font!");
        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let mut viewport = glyphon::Viewport::new(device, &cache);

        let mut atlas =
            glyphon::TextAtlas::new(device, queue, &cache, TextureFormat::Bgra8UnormSrgb);
        let text_renderer =
            TextRenderer::new(&mut atlas, device, MultisampleState::default(), None);

        Self {
            text_renderer,
            swash_cache,
            cache,
            atlas,
            font_system,
            viewport,
        }
    }
}

/// A single rectangle of text that can be rendered to the screen.
/// TODO: Use this from the outside world and convert to the glyphon format.
pub struct TextArea {
    buffer: glyphon::Buffer,
    rect: Rect<f32, f32>,
}
