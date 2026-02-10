use crate::app::composition::algebra::{Bubble, Empty};
use crate::app::composition::effects::{
    EffectHandler, ElementEffect, ElementEffectNode,
};
use crate::app::composition::elements::{Blueprint, Element};
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
    fn apply(&self, _: &mut TerminalEnvironment) {
        println!("[Terminal] Handling a RenderQuad!!!")
    }
}
impl<Env> ElementEffectNode<Env> for RenderQuad
where
    RenderQuad: ElementEffect<Env>,
{
    fn visit_with<Handler>(&self, h: &mut Handler)
    where
        Handler: EffectHandler<Env>,
    {
        h.handle_one(self);
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
    type Effect = RenderQuad;

    fn effect(&self) -> Self::Effect {
        RenderQuad(self.rect, self.color)
    }
}
