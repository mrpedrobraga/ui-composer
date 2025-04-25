use std::iter::repeat_with;

use super::{GPURenderer, RendererBuffers, Renderers};
use crate::gpu::backend::GPUResources;
use crate::gpu::render_target::GPURenderTarget;
use crate::prelude::UIItem;
use glyphon::{
    Cache, FontSystem, Resolution, SwashCache, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use vek::{Extent2, Rect};
use wgpu::{hal::auxil::db, ColorTargetState, MultisampleState, RenderPass, Texture, TextureFormat};

pub struct TextPipelineBuffers {
    buffers: Vec<cosmic_text::Buffer>,
}

impl TextPipelineBuffers {
    pub fn new(gpu_resources: &GPUResources, text_buffer_count: usize, renderer: &mut GlyphonTextRenderer) -> Self {
        let new_buffer = || {
            let mut buffer =
            cosmic_text::Buffer::new(&mut renderer.font_system, cosmic_text::Metrics::new(16.0, 20.0));

            buffer.set_text(
                &mut renderer.font_system,
                "Hello there!",
                cosmic_text::Attrs::new().family(cosmic_text::Family::Name("Work Sans")),
                cosmic_text::Shaping::Advanced,
            );
            buffer.set_size(
                &mut renderer.font_system,
                Some(100.0),
                Some(100.0),
            );
            buffer.set_wrap(&mut renderer.font_system, cosmic_text::Wrap::Word);
            buffer.shape_until_scroll(&mut renderer.font_system, false);
            buffer
        };

        let mut buffers = Vec::with_capacity(text_buffer_count);
        buffers.extend(repeat_with(new_buffer).take(text_buffer_count));

        Self {
            buffers
        }
    }
}

/// The pipeline for rendering text.
pub struct GlyphonTextRenderer {
    text_renderer: TextRenderer,
    font_system: FontSystem,
    viewport: Viewport,
    atlas: TextAtlas,
    cache: Cache,
    swash_cache: SwashCache,
}

impl GPURenderer for GlyphonTextRenderer {
    fn draw(
        gpu_resources: &mut GPUResources,
        pipelines: &mut Renderers,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        ui_tree: &mut dyn UIItem,
        render_buffers: &mut RendererBuffers,
    ) {
        let this = &mut pipelines.text_renderer;

        this.viewport.update(
            &gpu_resources.queue,
            Resolution {
                width: texture.width(),
                height: texture.height(),
            },
        );

        let text_areas = render_buffers.text_render_buffers.buffers.iter().map(|buffer| {
            glyphon::TextArea {
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
            }
        });

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

impl GlyphonTextRenderer {
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
