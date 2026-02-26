use glamour::{Size2, Vector2};

/// Offsets for the four sides of a rectangle.
/// Useful for margin, padding and other layout things.
#[repr(C)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct SideOffsets<Scalar = f32> {
    top: Scalar,
    bottom: Scalar,
    left: Scalar,
    right: Scalar,
}

pub trait RectExt {
    type Scalar: num_traits::Num + Clone;

    /// Expands this rectangle keeping its center in the same place, some amount from each face.
    /// This is useful for adding paddings to items inside layout.
    fn offset(self, offsets: SideOffsets<Self::Scalar>) -> Self;

    /// Expands this rectangle keeping its center in the same place, the same amount for every face.
    /// This is useful for adding paddings to items inside layout.
    fn expand(self, offset: Self::Scalar) -> Self
    where
        Self: Sized,
        Self::Scalar: std::ops::Neg<Output = Self::Scalar>,
    {
        self.offset(SideOffsets {
            top: -offset.clone(),
            bottom: offset.clone(),
            left: -offset.clone(),
            right: offset,
        })
    }

    /// Sets this rectangle (keeping the top left in place) to have a specific, definite size.
    fn with_size(self, size: Size2) -> Self;

    /// Translates this rectangle in space.
    fn translated(self, vector: Vector2) -> Self;
}
