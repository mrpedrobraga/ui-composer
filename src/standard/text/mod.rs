use crate::{
    app::render_state::RenderTarget,
    render_module::{IntoRenderModule, RenderModule},
};
use glyphon::{
    Attrs, Buffer, Cache, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer, Viewport,
};
use wgpu::MultisampleState;

pub struct TextRenderModule<'window> {
    buffer: glyphon::Buffer,
    text_renderer: TextRenderer,
    font_system: FontSystem,
    swash_cache: SwashCache,
    viewport: Viewport,
    atlas: TextAtlas,
    render_stuff: TextRenderStuff<'window>,
}

struct TextRenderStuff<'window> {
    /// This needs to be moved to `RenderState`, i.e., a pipeline doesn't know its own ID.
    pub id: u8,
    pub render_texture: RenderTarget<'window>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
}

pub struct Text<T: AsRef<str>>(pub T);

impl<T> IntoRenderModule for Text<T>
where
    T: AsRef<str>,
{
    fn into_render_module<'window>(
        &self,
        window: std::sync::Arc<winit::window::Window>,
        surface: wgpu::Surface<'window>,
        adapter: wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue,
    ) -> Box<dyn RenderModule + 'window> {
        let surface_config = surface
            .get_default_config(
                &adapter,
                window.inner_size().width,
                window.inner_size().height,
            )
            .unwrap();
        surface.configure(&device, &surface_config);

        let render_texture = RenderTarget {
            size: window.inner_size(),
            surface,
            surface_config,
        };

        let render_pipeline = TextRenderStuff {
            id: 2,
            render_texture,
            device,
            queue,
            adapter,
        };

        // Set up text renderer
        let mut font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(&render_pipeline.device);
        let viewport = Viewport::new(&render_pipeline.device, &cache);
        let mut atlas = TextAtlas::new(
            &render_pipeline.device,
            &render_pipeline.queue,
            &cache,
            render_pipeline.render_texture.surface_config.format,
        );
        let text_renderer = TextRenderer::new(
            &mut atlas,
            &render_pipeline.device,
            MultisampleState::default(),
            None,
        );
        let mut buffer = Buffer::new(&mut font_system, Metrics::new(30.0, 42.0));
        buffer.set_wrap(&mut font_system, glyphon::Wrap::Word);

        let attrs = Attrs::new().family(Family::SansSerif);

        let rich_text = &[(self.0.as_ref(), Attrs::new().weight(glyphon::Weight::BOLD))];

        buffer.set_size(&mut font_system, Some(400.0), Some(800.0));
        buffer.set_rich_text(
            &mut font_system,
            rich_text.iter().copied(),
            attrs,
            Shaping::Advanced,
        );
        buffer.shape_until_scroll(&mut font_system, false);

        Box::new(TextRenderModule {
            buffer,
            text_renderer,
            render_stuff: render_pipeline,
            atlas,
            font_system,
            swash_cache,
            viewport,
        })
    }
}

impl<'window> RenderModule for TextRenderModule<'window> {
    fn create_render_frame(&self) -> (wgpu::SurfaceTexture, wgpu::TextureView) {
        let surface_texture = self
            .render_stuff
            .render_texture
            .surface
            .get_current_texture()
            .unwrap();
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        return (surface_texture, view);
    }

    fn prepare<'pass>(&mut self, _current_pipeline_id: &mut Option<u8>) {
        let size = self.render_stuff.render_texture.size;

        self.text_renderer
            .prepare(
                &self.render_stuff.device,
                &self.render_stuff.queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                [TextArea {
                    buffer: &self.buffer,
                    left: 16.0,
                    top: 0.0,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: 0 + size.width as i32,
                        bottom: 0 + size.height as i32,
                    },
                    default_color: glyphon::Color::rgb(255, 255, 255),
                }],
                &mut self.swash_cache,
            )
            .unwrap();
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.render_stuff.render_texture.resize(
            &self.render_stuff.device,
            &self.render_stuff.adapter,
            new_size,
        );
        self.buffer.set_size(
            &mut self.font_system,
            Some(new_size.width as f32),
            Some(new_size.height as f32),
        );
        self.viewport.update(
            &self.render_stuff.queue,
            Resolution {
                width: new_size.width,
                height: new_size.height,
            },
        );
    }

    fn draw<'pass>(&'pass self, render_pass: &mut wgpu::RenderPass<'pass>) {
        self.text_renderer
            .render(&self.atlas, &self.viewport, render_pass)
            .unwrap();
    }

    fn get_command_encoder(&self) -> wgpu::CommandEncoder {
        self.render_stuff
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
    }

    fn present(&self, encoder: wgpu::CommandEncoder) {
        self.render_stuff.queue.submit(Some(encoder.finish()));
    }
}
