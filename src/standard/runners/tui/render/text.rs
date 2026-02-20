use crate::app::composition::algebra::{Bubble, Empty};
use crate::app::composition::effects::ElementEffect;
use crate::app::composition::elements::{Blueprint, Element};
use crate::app::composition::visit::{Apply, DriveThru};
use crate::prelude::hints::ParentHints;
use crate::prelude::{Event, LayoutItem};
use crate::runners::tui::nodes::TerminalEffectVisitor;
use crate::runners::tui::render::canvas::{Canvas, TextModePixel};
use crate::runners::tui::runner::TerminalEnvironment;
use crate::runners::winit::runner::WinitEnvironment;
use vek::{Extent2, Rect, Rgba};

/// An effect that describes rendering some text in the terminal.
#[derive(Debug)]
pub struct RenderText(pub Rect<f32, f32>, pub String, pub Rgba<f32>);

impl ElementEffect<WinitEnvironment> for RenderText {}

impl ElementEffect<TerminalEnvironment> for RenderText {}

impl<'fx> Apply<RenderText> for TerminalEffectVisitor<'fx> {
    fn visit(&mut self, RenderText(rect, text, color): &RenderText) {
        let rect = rect.as_::<usize, usize>();

        let mut curr_x = 0;
        let mut curr_y = 0;

        for word in text.split_whitespace() {
            let word_length = word.chars().count();

            // If the word won't fit because it's too big for the space left,
            // move the cursor to the start of the next line.
            if curr_x > 0 && curr_x + word_length > rect.w {
                curr_x = 0;
                curr_y += 1;
            }

            // If we're out of lines, stop drawing!
            if curr_y >= rect.h {
                return;
            }

            // Then we draw the word character by character, wrapping lines if it doesn't fit.
            // This wrapping behaviour is a fallback for words which are themselves too big to ever fit.
            // "Supercalifragilisticexpialidocius."
            for c in word.chars() {
                if curr_x >= rect.w {
                    curr_x = 0;
                    curr_y += 1;
                    if curr_y >= rect.h {
                        return;
                    }
                }

                self.canvas.put_pixel(
                    vek::Vec2::new(rect.x + curr_x, rect.y + curr_y),
                    TextModePixel {
                        bg_color: Rgba::new_transparent(0.0, 0.0, 0.0),
                        fg_color: *color,
                        character: c,
                    },
                );
                curr_x += 1;
            }

            // Put some whitespace between words if possible :-)
            if curr_x < rect.w {
                self.canvas.put_pixel(
                    vek::Vec2::new(rect.x + curr_x, rect.y + curr_y),
                    TextModePixel {
                        bg_color: Rgba::new_transparent(0.0, 0.0, 0.0),
                        fg_color: *color,
                        character: ' ',
                    },
                );
                curr_x += 1;
            }
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

impl LayoutItem for &'static str {
    type Blueprint = Text;

    fn get_natural_size(&self) -> Extent2<f32> {
        Extent2::new(15.0, 1.0)
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        Extent2::new(15.0, 1.0)
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::Blueprint {
        Text()
            .with_text(self.to_string())
            .with_rect(parent_hints.rect)
            .with_color(Rgba::white())
    }
}
