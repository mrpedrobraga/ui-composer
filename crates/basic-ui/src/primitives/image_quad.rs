use std::sync::Arc;

use image::{DynamicImage, GenericImageView};
use ui_composer_core::app::composition::{
    algebra::{Bubble, Empty},
    effects::ElementEffect,
    elements::Element,
    visit::{Apply, DriveThru},
};
use ui_composer_input::event::Event;
use ui_composer_math::prelude::{Rect, Srgba};
use ui_composer_platform_tui::{
    canvas::{Canvas as _, TextModePixel},
    nodes::TerminalEffectVisitor,
    runner::TerminalEnvironment,
};

use crate::components::ImageViewBlueprint;

/// An effect that describes rendering of a quad in the terminal.
#[derive(Debug)]
pub struct RenderImageQuad(
    pub Rect,
    // TODO: Maybe use a & instead of an arc here.
    pub std::sync::Arc<DynamicImage>,
);

//impl ElementEffect<WinitEnvironment> for RenderImageQuad {}

impl ElementEffect<TerminalEnvironment> for RenderImageQuad {}
impl<'fx> Apply<RenderImageQuad> for TerminalEffectVisitor<'fx> {
    fn visit(&mut self, RenderImageQuad(rect, image): &RenderImageQuad) {
        self.canvas.quad(rect.as_(), |input| {
            let (w, h) = image.dimensions();
            let (w, h) = (w as f32, h as f32);
            let (x, y) = (input.uv.x * w, input.uv.y * h);
            let (x, y) = (x as u32, y as u32);
            let pixel = image.get_pixel(x, y);
            TextModePixel {
                bg_color: Srgba::new(
                    pixel.0[0] as f32,
                    pixel.0[1] as f32,
                    pixel.0[2] as f32,
                    pixel.0[3] as f32,
                ) / 255.0, // TODO: Verify if I really need to normalize it.
                fg_color: Srgba::new(0.0, 0.0, 0.0, 0.0),
                character: ' ',
            }
        });
    }
}
impl Apply<RenderImageQuad> for () {
    fn visit(&mut self, _: &RenderImageQuad) {
        /* Do nothing for now */
    }
}
impl<V> DriveThru<V> for RenderImageQuad
where
    V: Apply<Self>,
{
    fn drive_thru(&self, visitor: &mut V) {
        visitor.visit(self);
    }
}

#[allow(non_snake_case)]
pub fn ImageView() -> ImageViewBlueprint {
    ImageViewBlueprint {
        rect: Rect::default(),
        image: Arc::new(DynamicImage::default()),
    }
}

/// A simple image
pub struct ImageViewElementTerminal {
    image: std::sync::Arc<DynamicImage>,
    rect: Rect,
}
impl Element<TerminalEnvironment> for ImageViewElementTerminal {
    type Effect<'fx>
        = RenderImageQuad
    where
        Self: 'fx;

    fn effect(&self) -> Self::Effect<'_> {
        RenderImageQuad(self.rect, self.image.clone())
    }
}

impl ImageViewElementTerminal {
    pub fn new(rect: Rect, image: std::sync::Arc<DynamicImage>) -> Self {
        Self { rect, image }
    }

    /// Adapts this graphic with a new colour!
    pub fn with_image(self, image: std::sync::Arc<DynamicImage>) -> Self {
        Self { image, ..self }
    }

    /// Adapts this graphic with a new rect!
    pub fn with_rect(self, rect: Rect) -> Self {
        Self { rect, ..self }
    }
}

impl Bubble<Event, bool> for ImageViewElementTerminal {
    fn bubble(&mut self, _: &mut Event) -> bool {
        Empty::empty()
    }
}
