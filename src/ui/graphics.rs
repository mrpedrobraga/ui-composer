use bytemuck::{Pod, Zeroable};
use std::mem::size_of;
use vek::{Aabr, Extent3, Mat4, Rect, Rgb, Vec2, Vec4};

use super::node::{LiveUINode, UINode};

/// A small fragment of graphics that can be sent to the GPU and rendered.
/// You can compose several primitives to make more impressive graphics.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Primitive {
    pub transform: Mat4<f32>,
    pub color: Rgb<f32>,
}

impl Primitive {
    pub fn rect(rect: Rect<f32, f32>, color: Rgb<f32>) -> Self {
        Self {
            transform: Mat4::identity()
                .scaled_3d(Extent3::new(rect.extent().w, rect.extent().h, 1.0))
                .translated_2d(rect.position()),
            color,
        }
    }
}

impl Default for Primitive {
    fn default() -> Self {
        Primitive {
            transform: Default::default(),
            color: Default::default(),
        }
    }
}

impl LiveUINode for Primitive {
    fn handle_ui_event(&mut self, event: super::node::UIEvent) -> bool {
        false
    }
}

impl UINode for Primitive {
    const PRIMITIVE_COUNT: usize = 1;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        // Beautifully calculating the bounds of this Primitive
        // as a Rect!
        let zero = self.transform * Vec4::zero();
        let one = self.transform * Vec4::one();
        Some(Rect::new(zero.x, zero.y, one.x - zero.x, one.y - zero.y))
    }
}
