use crate::elements::{Blueprint, Element, ElementEffect, DummyEnvironment};
use vek::{Extent3, Mat4, Rect, Rgba, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct RenderQuad {
    pub transform: Mat4<f32>,
    pub color: Rgba<f32>,
}

impl ElementEffect for RenderQuad {}

/// Represents a little rectangle to be drawn on screen!
#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    rect: Rect<f32, f32>,
    color: Rgba<f32>,
}

impl Rectangle {
    pub fn new(rect: Rect<f32, f32>, color: Rgba<f32>) -> Self {
        Self { rect, color }
    }
}

impl Blueprint<DummyEnvironment> for Rectangle {
    type Element = Self;

    fn spawn(self, env: &DummyEnvironment) -> Self::Element {
        self
    }
}

impl<Env> Element<Env> for Rectangle {
    type Effect = RenderQuad;
    fn effect(&self) -> Self::Effect {
        RenderQuad {
            transform: Mat4::identity()
                .scaled_3d(Extent3::new(
                    self.rect.extent().w,
                    self.rect.extent().h,
                    1.0,
                ))
                .translated_2d(self.rect.position())
                .translated_3d(Vec3::new(0.0, 0.0, 0.5)),
            color: self.color,
        }
    }
}
