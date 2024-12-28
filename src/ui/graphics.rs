use bytemuck::{Pod, Zeroable};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use vek::{Extent3, Mat4, Rect, Rgb, Vec3, Vec4};

use super::node::{ItemDescriptor, UIItem};

/// A small fragment of graphics that can be sent to the GPU and rendered.
/// You can compose several primitives to make more impressive graphics.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Graphic {
    pub transform: Mat4<f32>,
    pub color: Rgb<f32>,
    pub corner_radii: Vec4<f32>,
}

impl Graphic {
    pub fn new(rect: Rect<f32, f32>, color: Rgb<f32>) -> Self {
        Self {
            transform: Mat4::identity()
                .scaled_3d(Extent3::new(rect.extent().w, rect.extent().h, 1.0))
                .translated_2d(rect.position()),
            color,
            corner_radii: Vec4::zero(),
        }
    }

    /// Returns this graphic rotated by some angle.
    pub fn rotated(self, angle_radians: f32) -> Self {
        Self {
            transform: self.transform
                * Mat4::identity()
                    .translated_3d(-Vec3::one() * 0.5)
                    .rotated_z(angle_radians)
                    .translated_3d(Vec3::one() * 0.5),
            ..self
        }
    }

    /// Returns this graphic rotated by some angle.
    pub fn with_corner_radii(self, corner_radii: Vec4<f32>) -> Self {
        Self {
            corner_radii,
            ..self
        }
    }
}

impl Default for Graphic {
    fn default() -> Self {
        Graphic {
            transform: Default::default(),
            color: Default::default(),
            corner_radii: Default::default(),
        }
    }
}

impl UIItem for Graphic {
    fn handle_ui_event(&mut self, event: super::node::UIEvent) -> bool {
        false
    }

    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        quad_buffer[0] = *self;
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}

impl ItemDescriptor for Graphic {
    const QUAD_COUNT: usize = 1;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        // Beautifully calculating the bounds of this Primitive
        // as a Rect!
        let zero = self.transform * Vec4::zero();
        let one = self.transform * Vec4::one();
        Some(Rect::new(zero.x, zero.y, one.x - zero.x, one.y - zero.y))
    }
}
