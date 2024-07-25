use super::render::{AllocationInfo, UIFragment};
use crate::{
    app::engine::RenderTarget,
    interaction::InteractorNode,
    prelude::Primitive,
    reaction::Reactor,
    render_module::{IntoRenderModule, RenderModule},
};
use glyphon::{
    Attrs, Buffer, Cache, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache,
    TextArea as GlyphonTextArea, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use std::{any::Any, path::PathBuf};
use vek::{Aabr, Vec2};
use wgpu::MultisampleState;

pub struct TextRenderModule<'window> {
    initial: bool,
    reactors: Vec<Option<Reactor>>,
    primitive_buffer_cpu: Vec<Primitive>,
    interactors: Vec<Box<dyn InteractorNode>>,
    text_areas: Vec<TextArea>,
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
}

struct TextArea {
    buffer: glyphon::Buffer,
    aabb: Aabr<i32>,
}

pub struct Text<T: AsRef<str>>(pub T);

impl<T> IntoRenderModule for Text<T>
where
    T: AsRef<str>,
{
    fn into_render_module<'a, 'window>(
        self,
        window: std::sync::Arc<winit::window::Window>,
        surface: wgpu::Surface<'window>,
        adapter: &'a wgpu::Adapter,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
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

        let render_stuff = TextRenderStuff {
            id: 2,
            render_texture,
        };

        // Set up text renderer
        let mut font_system = FontSystem::new();
        font_system
            .db_mut()
            .load_font_file("./Source_Sans_3/SourceSans3-Regular.ttf");

        let swash_cache = SwashCache::new();
        let cache = Cache::new(&device);
        let viewport = Viewport::new(&device, &cache);
        let mut atlas = TextAtlas::new(
            &device,
            &queue,
            &cache,
            render_stuff.render_texture.surface_config.format,
        );
        let text_renderer =
            TextRenderer::new(&mut atlas, &device, MultisampleState::default(), None);
        let mut text_buffer = Buffer::new(&mut font_system, Metrics::new(16.0, 16.0 * 1.2));
        text_buffer.set_wrap(&mut font_system, glyphon::Wrap::Word);
        let attrs = Attrs::new().family(Family::Name("Source Sans 3"));
        let rich_text = &[(self.0.as_ref(), Attrs::new())];

        text_buffer.set_size(&mut font_system, Some(400.0), Some(800.0));
        text_buffer.set_rich_text(
            &mut font_system,
            rich_text.iter().copied(),
            attrs,
            Shaping::Advanced,
        );
        text_buffer.shape_until_scroll(&mut font_system, false);
        let text_area = TextArea {
            buffer: text_buffer,
            aabb: Aabr {
                min: Vec2::new(0, 0),
                max: Vec2::new(
                    window.inner_size().width as i32,
                    window.inner_size().height as i32,
                ),
            },
        };
        let text_areas = vec![text_area];

        Box::new(TextRenderModule {
            initial: true,
            text_areas,
            text_renderer,
            render_stuff,
            atlas,
            font_system,
            swash_cache,
            viewport,
            reactors: Vec::new(),
            primitive_buffer_cpu: Vec::new(),
            interactors: Vec::new(),
        })
    }
}

impl<'window> RenderModule for TextRenderModule<'window> {
    fn initial(&self) -> bool {
        self.initial
    }

    fn set_setup_finished(&mut self) {
        self.initial = false;
    }

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

    fn resize(
        &mut self,
        new_size: winit::dpi::PhysicalSize<u32>,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        adapter: &wgpu::Adapter,
    ) {
        self.render_stuff
            .render_texture
            .resize(&device, &adapter, new_size);
        self.viewport.update(
            &queue,
            Resolution {
                width: new_size.width,
                height: new_size.height,
            },
        );
    }

    // There are no events to be handled in this module
    fn handle_event(&mut self, _: winit::event::WindowEvent) -> bool {
        false
    }

    fn get_command_encoder(&self, device: &wgpu::Device) -> wgpu::CommandEncoder {
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
    }

    fn draw<'pass>(
        &'pass mut self,
        current_pipeline_id: &mut Option<u8>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'pass>,
    ) {
        let size = self.render_stuff.render_texture.size;

        /// Prepare!
        self.text_renderer
            .prepare(
                &device,
                &queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                self.text_areas.iter().map(|ta| GlyphonTextArea {
                    buffer: &ta.buffer,
                    left: ta.aabb.min.x as f32,
                    top: ta.aabb.min.y as f32,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: ta.aabb.min.x,
                        top: ta.aabb.min.y,
                        right: ta.aabb.max.x,
                        bottom: ta.aabb.max.y,
                    },
                    default_color: glyphon::Color::rgb(0xFF, 0xFF, 0xFF),
                }),
                &mut self.swash_cache,
            )
            .unwrap();

        /// Draw!
        self.text_renderer
            .render(&self.atlas, &self.viewport, render_pass)
            .unwrap();
    }

    fn present(&self, queue: &wgpu::Queue, encoder: wgpu::CommandEncoder) {
        queue.submit(Some(encoder.finish()));
    }

    fn reactors(&mut self) -> &mut Vec<Option<Reactor>> {
        &mut self.reactors
    }

    fn primitive_buffer(&mut self) -> &mut Vec<crate::prelude::Primitive> {
        &mut self.primitive_buffer_cpu
    }

    fn interactors(&mut self) -> &mut Vec<Box<dyn InteractorNode>> {
        &mut self.interactors
    }
}

// impl<T> UIFragment for Text<T>
// where
//     T: AsRef<str>,
// {
//     fn get_allocation_info() -> super::render::AllocationInfo {
//         AllocationInfo {
//             buffer_size: 0,
//             primitive_count: 0,
//         }
//     }

//     fn push_allocation(
//         self,
//         render_module: &mut super::render::tuple_render_module::TupleRenderModule,
//     ) {
//         render_module
//             .sub_modules
//             .push(self.into_render_module(window, surface, adapter, device, queue))
//     }
// }
