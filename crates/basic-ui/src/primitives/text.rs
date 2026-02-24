use ui_composer_core::app::composition::{
    algebra::{Bubble, Empty},
    effects::ElementEffect,
    elements::{Blueprint, Element},
    visit::{Apply, DriveThru},
};
use ui_composer_input::event::Event;
use ui_composer_platform_tui::{
    canvas::{Canvas as _, TextModePixel},
    nodes::TerminalEffectVisitor,
    runner::TerminalEnvironment,
};
use ui_composer_platform_winit::runner::WinitEnvironment;
use vek::{Rect, Rgba};

/// An effect that describes rendering some text in the terminal.
#[derive(Debug)]
pub struct RenderText(pub Rect<f32, f32>, pub String, pub Rgba<f32>);

impl ElementEffect<WinitEnvironment> for RenderText {}

impl ElementEffect<TerminalEnvironment> for RenderText {}

impl<'fx> Apply<RenderText> for TerminalEffectVisitor<'fx> {
    fn visit(&mut self, RenderText(rect, text, color): &RenderText) {
        let rect = rect.as_::<usize, usize>();

        for (i, ch) in text.chars().enumerate() {
            self.canvas.put_pixel(
                vek::Vec2::new(rect.x + i, rect.y),
                TextModePixel {
                    bg_color: Rgba::new_transparent(0.0, 0.0, 0.0),
                    fg_color: *color,
                    character: ch,
                },
            );
        }
    }
}
impl<V> DriveThru<V> for RenderText
where
    V: Apply<Self>,
{
    fn drive_thru(&self, visitor: &mut V) {
        visitor.visit(self);
    }
}

pub fn Text() -> Text {
    Text {
        rect: Rect::default(),
        text: String::new(),
        color: Rgba::default(),
    }
}

/// A simple coloured graphic.
#[derive(Default, Clone, PartialEq)]
pub struct Text {
    pub rect: Rect<f32, f32>,
    pub text: String,
    pub color: Rgba<f32>,
}

impl Text {
    /// Adapts this Text with a new colour!
    pub fn with_color(self, color: Rgba<f32>) -> Self {
        Self { color, ..self }
    }

    /// Adapts this Text with a new rect!
    pub fn with_rect(self, rect: Rect<f32, f32>) -> Self {
        Self { rect, ..self }
    }

    /// Adapts this Text with a new text content.
    pub fn with_text(self, text: String) -> Self {
        Self { text, ..self }
    }
}

impl Bubble<Event, bool> for Text {
    fn bubble(&mut self, _: &mut Event) -> bool {
        Empty::empty()
    }
}

impl Blueprint<TerminalEnvironment> for Text {
    type Element = Self;

    fn make(self, _: &TerminalEnvironment) -> Self::Element {
        self
    }
}

impl Element<TerminalEnvironment> for Text {
    type Effect<'fx> = RenderText;

    fn effect(&self) -> Self::Effect<'_> {
        RenderText(self.rect, self.text.clone(), self.color)
    }
}
