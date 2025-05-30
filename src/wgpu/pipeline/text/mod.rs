use crate::wgpu::backend::GPUResources;
use crate::wgpu::render_target::{Render, RenderTarget};
use glyphon::cosmic_text::Align;
use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Wrap};
use wgpu::{CompareFunction, DepthStencilState};
use {
    super::{GPURenderer, RendererBuffers, Renderers},
    glyphon::{
        Cache, FontSystem, Resolution, SwashCache, TextAtlas, TextBounds, TextRenderer, Viewport,
        Weight,
    },
    vek::{Extent2, Rect, Rgb},
    wgpu::{ColorTargetState, MultisampleState, RenderPass, Texture, TextureFormat},
};

#[doc(hidden)]
pub mod implementations;

/// Trait that describes something that can render to this text pipeline.
pub trait RenderText {
    /// Yields a text area to draw.
    fn push_text<'a>(&'a self, bounds: TextBounds, container: &mut Vec<glyphon::TextArea<'a>>);
}

pub struct Text(pub Rect<f32, f32>, pub String, pub Rgb<f32>);

pub struct TextItem {
    pub rect: Rect<f32, f32>,
    pub buffer: Buffer,
    pub color: Rgb<f32>,
}

pub struct TextPipelineBuffers {}

impl TextPipelineBuffers {
    pub fn new(
        _gpu_resources: &GPUResources,
        #[expect(unused)] renderer: &mut GlyphonTextRenderer,
    ) -> Self {
        Self {}
    }
}

#[allow(unused)]
fn default_buffer(renderer: &mut GlyphonTextRenderer) -> Buffer {
    let mut buffer = Buffer::new(&mut renderer.font_system, Metrics::new(16.0, 20.0));

    buffer.set_text(
        &mut renderer.font_system,
        "Click me...",
        &Attrs::new()
            .family(Family::Name("Work Sans"))
            .weight(Weight::NORMAL),
        Shaping::Advanced,
    );
    buffer.lines[0].set_align(Some(Align::Center));
    buffer.set_size(&mut renderer.font_system, Some(100.0), Some(100.0));
    buffer.set_wrap(&mut renderer.font_system, Wrap::Word);
    buffer.shape_until_scroll(&mut renderer.font_system, false);
    buffer
}

/// The pipeline for rendering text.
pub struct GlyphonTextRenderer {
    text_renderer: TextRenderer,
    font_system: FontSystem,
    viewport: Viewport,
    atlas: TextAtlas,
    //cache: Cache,
    swash_cache: SwashCache,
}

impl GPURenderer for GlyphonTextRenderer {
    fn draw<'draw, R: Render>(
        gpu_resources: &mut GPUResources,
        pipelines: &mut Renderers,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        ui_tree: &'draw R,
        _render_buffers: &mut RendererBuffers,
    ) {
        let this = &mut pipelines.text_renderer;

        this.viewport.update(
            &gpu_resources.queue,
            Resolution {
                width: texture.width(),
                height: texture.height(),
            },
        );

        let mut container = vec![];
        ui_tree.push_text(
            TextBounds {
                left: 0,
                top: 0,
                right: render_target_size.w as i32,
                bottom: render_target_size.h as i32,
            },
            &mut container,
        );

        this.text_renderer
            .prepare(
                &gpu_resources.device,
                &gpu_resources.queue,
                &mut this.font_system,
                &mut this.atlas,
                &this.viewport,
                container.iter().cloned(),
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
        Target: RenderTarget,
    {
        let _ = adapter;
        let _ = render_target_formats;
        let mut font_system = FontSystem::new();
        font_system
            .db_mut()
            .load_font_file("./assets/WorkSans-Regular.ttf")
            .expect("Failed to load font!");
        font_system
            .db_mut()
            .load_font_file("./assets/Anima Sans.ttf")
            .expect("Failed to load font!");
        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let viewport = glyphon::Viewport::new(device, &cache);

        let mut atlas =
            glyphon::TextAtlas::new(device, queue, &cache, TextureFormat::Bgra8UnormSrgb);
        let text_renderer = TextRenderer::new(
            &mut atlas,
            device,
            MultisampleState::default(),
            Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: Default::default(),
                bias: Default::default(),
            }),
        );

        Self {
            text_renderer,
            swash_cache,
            //cache,
            atlas,
            font_system,
            viewport,
        }
    }
}
