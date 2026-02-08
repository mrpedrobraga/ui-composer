//! # Extensions
//!
//! Extension traits to make working with some foreign types a little bit more ergonomic.
use super::BaseNum;
use super::{Extent2, Extent3, Rect, Vec2, Vec3};

pub trait RectExt {
    type Num: BaseNum;

    /// Expands this rectangle keeping its center in the same place, some amount from each face.
    /// This is useful for adding paddings to items inside layout.
    fn expand_from_center(
        self,
        left: Self::Num,
        right: Self::Num,
        top: Self::Num,
        bottom: Self::Num,
    ) -> Self;

    /// Expands this rectangle keeping its center in the same place, the same amount for every face.
    /// This is useful for adding paddings to items inside layout.
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
}
