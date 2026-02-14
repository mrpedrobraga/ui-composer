use std::io::Write;

use crate::app::composition::algebra::{Bubble, Empty};
use crate::app::composition::effects::ElementEffect;
use crate::app::composition::elements::{Blueprint, Element};
use crate::app::composition::visit::{Apply, DriveThru};
use crate::geometry::Lerp;
use crate::runners::tui::nodes::TerminalEffectVisitor;
use crate::runners::tui::render::canvas::{Canvas, TextModePixel};
use crate::runners::tui::runner::TerminalEnvironment;
use crate::runners::winit::runner::WinitEnvironment;
use vek::Rect;
use {crate::app::input::Event, vek::Rgba};

pub mod canvas;
pub mod shaders;

/// An effect that describes rendering of a quad in the terminal.
#[derive(Debug)]
pub struct RenderQuad(pub Rect<f32, f32>, pub Rgba<f32>);
impl ElementEffect<WinitEnvironment> for RenderQuad {
    fn apply(&self, _: &mut WinitEnvironment) {
        println!("[Winit] Handling a RenderQuad!!!");
    }
}
impl ElementEffect<TerminalEnvironment> for RenderQuad {
    fn apply(&self, env: &mut TerminalEnvironment) {
        /* TODO: Remove this function */
    }
}
impl<'fx> Apply<RenderQuad> for TerminalEffectVisitor<'fx> {
    fn visit(&mut self, RenderQuad(rect, color): &RenderQuad) {
        self.canvas.rect(
            rect.as_(),
            TextModePixel {
                bg_color: *color,
                fg_color: Rgba::zero(),
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

pub fn Graphic() -> Graphic {
    Graphic {
        rect: Rect::default(),
        color: Rgba::default(),
    }
}

/// A simple coloured graphic.
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Graphic {
    pub rect: Rect<f32, f32>,
    pub color: Rgba<f32>,
}

impl From<Rect<f32, f32>> for Graphic {
    fn from(value: Rect<f32, f32>) -> Self {
        Graphic {
            rect: value,
            ..Default::default()
        }
    }
}

impl Graphic {
    /// Adapts this graphic with a new colour!
    pub fn with_color(self, color: Rgba<f32>) -> Self {
        Self { color, ..self }
    }

    /// Adapts this graphic with a new rect!
    pub fn with_rect(self, rect: Rect<f32, f32>) -> Self {
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

    fn make(self, _: &TerminalEnvironment) -> Self::Element {
        self
    }
}

impl Element<TerminalEnvironment> for Graphic {
    type Effect<'fx> = RenderQuad;

    fn effect(&self) -> Self::Effect<'_> {
        RenderQuad(self.rect, self.color)
    }
}

impl Lerp for Graphic {
    fn linear_interpolate(self, other: Self, t: f32) -> Self {
        Graphic {
            rect: Rect {
                x: self.rect.x.linear_interpolate(other.rect.x, t),
                y: self.rect.y.linear_interpolate(other.rect.y, t),
                w: self.rect.w.linear_interpolate(other.rect.w, t),
                h: self.rect.h.linear_interpolate(other.rect.h, t),
            },
            color: Rgba {
                r: self.color.r.linear_interpolate(other.color.r, t),
                g: self.color.g.linear_interpolate(other.color.g, t),
                b: self.color.b.linear_interpolate(other.color.b, t),
                a: self.color.a.linear_interpolate(other.color.a, t),
            },
        }
    }
}
