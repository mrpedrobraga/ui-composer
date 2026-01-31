//! # Algebra
//!
//! This module contains a trait that defines a bidirectional
//! structural algebra with a fold-unfold operation (bubble).

use std::mem::MaybeUninit;

pub mod implementations;

pub trait Bubble<Down, Up> {
    fn bubble(&mut self, cx: &mut Down) -> Up;
}

/// Like [`Bubble`] but combines the result into a cartesian product
/// (a buffer) instead of using monoids.
pub trait Gather<Context, Item> {
    const SIZE: usize;

    fn gather(&mut self, cx: &mut Context, acc: &mut [MaybeUninit<Item>]);
}

#[cfg(feature = "specialization")]
impl<T, Down, Up> Bubble<Down, Up> for T where Up: Empty {
    default fn bubble(&mut self, cx: &mut Down) -> Up {
        Up::empty()
    }
}

/// Trait for "combining" two things into one thing.
///
/// The trait assumes that `combine` is associative,
/// that is, `combine(a, combine(b, c))` is the same as
/// `combine(combine(a, b), c)`.
pub trait Semigroup {
    /// Combines `self` and `other`.
    fn combine(self, other: Self) -> Self;
}

/// A [`Semigroup`] that also has an identity (null) element.
pub trait Empty {
    /// Returns the identity element.
    fn empty() -> Self;
}

pub trait Monoid: Semigroup + Empty {}
impl<T> Monoid for T where T: Semigroup + Empty {}