use {
    super::{RenderText, Text},
    crate::{
        app::node::{AppItem, UIEvent},
        winitwgpu::pipeline::graphics::{RenderGraphic, RenderGraphicDescriptor},
    },
    glyphon::{Color, TextArea, TextBounds},
    std::{
        pin::Pin,
        task::{Context, Poll},
    },
};

impl RenderText for Text {
    fn push_text<'a>(
        &self,
        buffer: &'a glyphon::Buffer,
        bounds: TextBounds,
        container: &mut Vec<TextArea<'a>>,
    ) {
        let v: vek::Rgb<u8> = (self.2 * 255.0).as_();

        container.push(glyphon::TextArea {
            buffer,
            left: self.0.x,
            top: self.0.y,
            scale: 1.0,
            bounds,
            default_color: Color::rgb(v.r, v.g, v.b),
            custom_glyphs: &[],
        });
    }
}

impl AppItem for Text {
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        false
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}

impl RenderGraphicDescriptor for Text {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        Some(self.0)
    }
}

impl RenderGraphic for Text {
    fn write_quads(&self, quad_buffer: &mut [crate::prelude::Graphic]) {}

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

impl RenderText for () {
    fn push_text<'a>(
        &self,
        buffer: &'a glyphon::Buffer,
        bounds: TextBounds,
        container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
    }
}

impl<A, B> RenderText for (A, B)
where
    A: RenderText,
    B: RenderText,
{
    fn push_text<'a>(
        &self,
        buffer: &'a glyphon::Buffer,
        bounds: TextBounds,
        container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
        self.0.push_text(buffer, bounds, container);
        self.1.push_text(buffer, bounds, container);
    }
}
