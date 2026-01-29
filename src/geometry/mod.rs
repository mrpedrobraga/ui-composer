//! Geometry API for quickly laying out items in space.
//!
//! It uses much of [`vek`] for basic vector implementations,
//! [`cgmath`] for numeric types, but has its own trait extensions
//! and implementations.

pub mod rect_ext;

/// Module for layout functions and utilities.
pub mod layout;

pub use cgmath::BaseNum;
pub use rect_ext::RectExt;
pub use vek::*;

use core::ops::{Add, Mul, Sub};
use vek::num_traits::{One, Zero};

/// A Vector is a value that be added to itself and be scaled.
pub trait Vector:
    Zero + One + Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<f32, Output = Self>
{
    fn linear_interpolate(self, other: Self, t: f32) -> Self {
        self * (1.0 - t) + other * t
    }
}

impl<T: Zero + One + Add<Output = Self> + Sub<Output = Self> + Mul<f32, Output = Self>> Vector
    for T
{
}
