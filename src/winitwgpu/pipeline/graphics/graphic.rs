use crate::{
    app::node::{AppItem, UIEvent},
    winitwgpu::pipeline::{
        graphics::{RenderGraphic, RenderGraphicDescriptor},
        text::RenderText,
    },
};
use bytemuck::{Pod, Zeroable};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use vek::{num_traits::AsPrimitive, Extent3, Mat4, Rect, Rgb, Vec3, Vec4};

/// A small fragment of graphics that can be sent to the GPU and rendered.
/// You can compose several primitives to make more impressive graphics.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, Default)]
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

    /// Returns this graphic with altered corner radii.
    pub fn with_corner_radii(self, corner_radii: Vec4<f32>) -> Self {
        Self {
            corner_radii,
            ..self
        }
    }

    /// Returns this graphic with a new colour.
    pub fn with_color(self, color: Rgb<f32>) -> Self {
        Self { color, ..self }
    }
}

impl<A, B> From<Rect<A, B>> for Graphic
where
    A: AsPrimitive<f32>,
    B: AsPrimitive<f32> + cgmath::One,
{
    fn from(value: Rect<A, B>) -> Self {
        Graphic {
            transform: Mat4::identity()
                .scaled_3d(Extent3::new(value.extent().w, value.extent().h, B::one()).as_())
                .translated_2d(value.position().as_()),
            ..Default::default()
        }
    }
}

impl AppItem for Graphic {
    fn handle_ui_event(&mut self, _event: UIEvent) -> bool {
        false
    }

    fn poll_processors(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}
