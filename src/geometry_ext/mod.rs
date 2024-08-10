use std::ops::{Add, Sub};
use vek::Rect;

pub trait RectExt {
    type Num: Copy + Add<Self::Num, Output = Self::Num> + Sub<Self::Num, Output = Self::Num>;

    fn expand_from_center(
        self,
        left: Self::Num,
        right: Self::Num,
        top: Self::Num,
        bottom: Self::Num,
    ) -> Self;

    fn expand_radius(self, offset: Self::Num) -> Self
    where
        Self: Sized,
    {
        self.expand_from_center(offset, offset, offset, offset)
    }
}

impl<T: Copy + Add<T, Output = T> + Sub<T, Output = T>> RectExt for Rect<T, T> {
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
}
