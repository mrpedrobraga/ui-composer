use ui_composer_core::app::composition::{
    algebra::{Empty, Propagate},
    effects::ElementEffect,
    elements::{Blueprint, Element},
    visit::{Apply, DriveThru},
};
use ui_composer_input::event::Event;
use ui_composer_math::{
    glamour::{Matrix4, Vector4},
    prelude::{Mix, Rect, Srgba},
};
use ui_composer_platform_tui::{
    canvas::{Canvas as _, TextModePixel},
    items::TerminalEffectVisitor,
    runner::{TerminalBlueprintResources, TerminalEnvironment},
};
use ui_composer_platform_winit::{
    runner::{DesktopEnvironment, DesktopResources},
    window::effect_handling::{QuadInstance, WindowEffectVisitor},
};

/// An effect that describes rendering of a quad in the terminal.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderQuad(pub Rect, pub Srgba);

//impl ElementEffect<WinitEnvironment> for RenderQuad {}
impl RenderQuad {
    pub fn as_quad_instance(&self) -> QuadInstance {
        QuadInstance {
            matrix: Matrix4::from_cols_array(&[
                // xx, xy, xz, xw,
                self.0.width(),
                0.0,
                0.0,
                0.0,
                // yx, yy, yz, yw,
                0.0,
                self.0.height(),
                0.0,
                0.0,
                // zx, zy, zz, zw,
                0.0,
                0.0,
                1.0,
                0.0,
                // wx, wy, wz, ww
                self.0.origin.x,
                self.0.origin.y,
                0.0,
                1.0,
            ]),
            color: Vector4 {
                x: self.1.red,
                y: self.1.green,
                z: self.1.blue,
                w: self.1.alpha,
            },
        }
    }
}

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

impl<'fx> Apply<RenderQuad> for WindowEffectVisitor<'fx> {
    fn visit(&mut self, render_quad: &RenderQuad) {
        /* Do nothing for now */
        self.quads.push(render_quad.as_quad_instance())
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
#[derive(Default, Debug, Clone, Copy, PartialEq)]
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

impl Propagate<Event, bool> for Graphic {
    async fn propagate(&mut self, _: &mut Event) -> bool {
        Empty::empty()
    }
}

impl Blueprint<TerminalEnvironment> for Graphic {
    type Output = Self;

    fn make(self, _: &TerminalBlueprintResources) -> Self::Output {
        self
    }
}

impl Element<TerminalEnvironment> for Graphic {
    type Effect = RenderQuad;

    fn effect(&self) -> Self::Effect {
        RenderQuad(self.rect, self.color)
    }

    type Blueprint = Self;

    fn update(&mut self, blueprint: Self, _: &TerminalBlueprintResources) {
        *self = blueprint
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

impl Blueprint<DesktopEnvironment> for Graphic {
    type Output = Self;

    fn make(self, _: &DesktopResources<'_>) -> Self::Output {
        self
    }
}

impl Element<DesktopEnvironment> for Graphic {
    type Effect = RenderQuad;

    fn effect(&self) -> Self::Effect {
        RenderQuad(self.rect, self.color)
    }

    type Blueprint = Self;

    fn update(&mut self, blueprint: Self, _: &DesktopResources) {
        *self = blueprint;
    }
}
