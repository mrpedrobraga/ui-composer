use ui_composer_core::app::composition::{
    algebra::{Bubble, Empty},
    effects::ElementEffect,
    elements::{Blueprint, Element},
    visit::{Apply, DriveThru},
};
use ui_composer_input::event::Event;
use ui_composer_math::prelude::{Mix, Rect, Srgba};
use ui_composer_platform_tui::{
    canvas::{Canvas as _, TextModePixel},
    nodes::TerminalEffectVisitor,
    runner::{TerminalBlueprintResources, TerminalEnvironment},
};

/// An effect that describes rendering of a quad in the terminal.
#[derive(Debug)]
pub struct RenderQuad(pub Rect, pub Srgba);

//impl ElementEffect<WinitEnvironment> for RenderQuad {}

impl ElementEffect<TerminalEnvironment> for RenderQuad {}
impl<'fx> Apply<RenderQuad> for TerminalEffectVisitor<'fx> {
    fn visit(&mut self, RenderQuad(rect, color): &RenderQuad) {
        self.canvas.rect(
            rect.as_(),
            TextModePixel {
                bg_color: *color,
                fg_color: Srgba::new(0.0, 0.0, 0.0, 0.0),
                character: ' ',
            },
        );
    }
}
impl Apply<RenderQuad> for () {
    fn visit(&mut self, _: &RenderQuad) {
        /* Do nothing for now */
    }
}
impl<V> DriveThru<V> for RenderQuad
where
    V: Apply<Self>,
{
    fn drive_thru(&self, visitor: &mut V) {
        visitor.visit(self);
    }
}

#[allow(non_snake_case)]
pub fn Graphic() -> Graphic {
    Graphic {
        rect: Rect::default(),
        color: Srgba::default(),
    }
}

/// A simple coloured graphic.
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Graphic {
    pub rect: Rect,
    pub color: Srgba,
}

impl From<Rect> for Graphic {
    fn from(value: Rect) -> Self {
        Graphic {
            rect: value,
            ..Default::default()
        }
    }
}

impl Graphic {
    pub fn new(rect: Rect, color: Srgba) -> Self {
        Self { rect, color }
    }

    /// Adapts this graphic with a new colour!
    pub fn with_color(self, color: Srgba) -> Self {
        Self { color, ..self }
    }

    /// Adapts this graphic with a new rect!
    pub fn with_rect(self, rect: Rect) -> Self {
        Self { rect, ..self }
    }
}

impl Bubble<Event, bool> for Graphic {
    fn bubble(&mut self, _: &mut Event) -> bool {
        Empty::empty()
    }
}

impl Blueprint<TerminalEnvironment> for Graphic {
    type Element = Self;

    fn make(self, _: &TerminalBlueprintResources) -> Self::Element {
        self
    }
}

impl Element<TerminalEnvironment> for Graphic {
    type Effect<'fx> = RenderQuad;

    fn effect(&self) -> Self::Effect<'_> {
        RenderQuad(self.rect, self.color)
    }
}

impl ui_composer_state::effect::animation::Lerp for Graphic {
    fn linear_interpolate(self, other: Self, t: f32) -> Self {
        Graphic {
            rect: self.rect.lerp(other.rect, t),
            color: self.color.mix(other.color, t),
        }
    }
}
