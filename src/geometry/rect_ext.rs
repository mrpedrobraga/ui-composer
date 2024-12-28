use super::BaseNum;
use super::{Extent2, Extent3, Mat4, Rect, Vec2, Vec3};
use crate::prelude::Graphic;
use vek::Vec4;

pub trait RectExt {
    type Num: BaseNum;

    /// Expands this rectangle keeping its center in the same place, some amount from each face.
    /// This is useful for adding paddings to items inside containers.
    fn expand_from_center(
        self,
        left: Self::Num,
        right: Self::Num,
        top: Self::Num,
        bottom: Self::Num,
    ) -> Self;

    /// Expands this rectangle keeping its center in the same place, the same amount for every face.
    /// This is useful for adding paddings to items inside containers.
    fn expand(self, offset: Self::Num) -> Self
    where
        Self: Sized,
    {
        self.expand_from_center(offset, offset, offset, offset)
    }

    /// Sets this rectangle (keeping the top left in place) to have a specific, definite size.
    fn with_size(self, size: Extent2<Self::Num>) -> Self;

    /// Translates this rectangle in space.
    fn translated(self, vector: Vec2<Self::Num>) -> Self;

    /// Transforms this rectangle into a renderable graphic.
    fn with_color(self, color: vek::Rgb<f32>) -> Graphic;
}

impl<T: BaseNum> RectExt for Rect<T, T>
where
    Vec2<f32>: From<Vec2<T>>,
    Vec3<f32>: From<Extent3<T>>,
{
    type Num = T;

    fn expand_from_center(
        self,
        left: Self::Num,
        right: Self::Num,
        top: Self::Num,
        bottom: Self::Num,
    ) -> Self {
        Rect {
            x: self.x - left,
            y: self.y - top,
            w: self.w + left + right,
            h: self.h + top + bottom,
        }
    }

    fn with_size(self, size: Extent2<Self::Num>) -> Self {
        Self {
            x: self.x,
            y: self.y,
            w: size.w,
            h: size.h,
        }
    }

    fn translated(self, vector: Vec2<Self::Num>) -> Self {
        Self {
            x: self.x + vector.x,
            y: self.y + vector.y,
            ..self
        }
    }

    /// Consumes the [`Rect`] by returning a coloured rectangular [`Graphic`].
    fn with_color(self, color: vek::Rgb<f32>) -> Graphic {
        Graphic {
            transform: Mat4::identity()
                .scaled_3d(Extent3::new(self.extent().w, self.extent().h, T::one()))
                .translated_2d(self.position()),
            color,
            corner_radii: Vec4::zero(),
        }
    }
}
