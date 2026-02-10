//! # Geometry
//!
//! Mathematical utilities for laying out things in space.

pub mod geometry_ext;

pub mod flow;

pub use geometry_ext::RectExt;
pub use vek::*;

use core::ops::{Add, Mul, Sub};
use vek::num_traits::{One, Zero};

/// A Vector is a value that be added to itself and be scaled.
pub trait Vector:
    Zero
    + One
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f32, Output = Self>
{
    fn linear_interpolate(self, other: Self, t: f32) -> Self {
        self * (1.0 - t) + other * t
    }
}

impl<
    T: Zero
        + One
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<f32, Output = Self>,
> Vector for T
{
}
