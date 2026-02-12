//! # Geometry
//!
//! Mathematical utilities for laying out things in space.

pub mod geometry_ext;

pub mod flow;

pub use geometry_ext::RectExt;
pub use vek::*;

use core::ops::{Add, Mul, Sub};
use num_traits::Num;
use vek::num_traits::{One, Zero};

/// A Vector is a value that be added to itself and be scaled.
pub trait Vector:
    Zero
    + One
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f32, Output = Self>
{
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

/// Two [LinearInterpolate]s can be smoothly transformed from one to the other.
pub trait Lerp {
    fn linear_interpolate(self, other: Self, t: f32) -> Self;
}

impl<T: Num + Mul<f32, Output = Self>> Lerp for T {
    fn linear_interpolate(self, other: Self, t: f32) -> Self  {
        self * (1.0 - t) + other * t
    }
}