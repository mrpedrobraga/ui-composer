use crate::app::primitives::Processor;
use {
    crate::app::{input::Event, primitives::Primitive},
    bytemuck::{Pod, Zeroable},
    vek::{Extent3, Mat4, Rect, Rgb, Vec3, Vec4, num_traits::AsPrimitive},
};

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

    /// Returns this graphic translated by a 3d vector.
    pub fn translated_3d(self, translation: Vec3<f32>) -> Self {
        Self {
            transform: self.transform.translated_3d(translation),
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
        let pos_2d = value.position().as_();

        Graphic {
            transform: Mat4::identity()
                .scaled_3d(Extent3::new(value.extent().w, value.extent().h, B::one()).as_())
                .translated_3d(Vec3::new(pos_2d.x, pos_2d.y, 0.5)),
            ..Default::default()
        }
    }
}

impl<Res> Primitive<Res> for Graphic {
    fn handle_event(&mut self, _event: Event) -> bool {
        false
    }
}

impl<Res> Processor<Res> for Graphic {}
