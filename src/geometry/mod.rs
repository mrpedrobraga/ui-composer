//! Geometry API for quickly laying out items in space.
//!
//! It uses much of [`vek`] for basic vector implementations,
//! [`cgmath`] for numeric types, but has its own trait extensions
//! and implementations.
pub mod rect_ext;

pub use cgmath::BaseNum;
pub use rect_ext::RectExt;
use vek::num_traits::{One, Zero};
pub use vek::*;

/// A Vector is a value that be added to itself and be scaled.
pub trait Vector:
    Zero
    + One
    + std::ops::Add<Self, Output = Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::Mul<f32, Output = Self>
{
    fn linear_interpolate(self, other: Self, t: f32) -> Self {
        self * (1.0 - t) + other * t
    }
}

impl<
        T: Zero
            + One
            + std::ops::Add<Output = Self>
            + std::ops::Sub<Output = Self>
            + std::ops::Mul<f32, Output = Self>,
    > Vector for T
{
}
