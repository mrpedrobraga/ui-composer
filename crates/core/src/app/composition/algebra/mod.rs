//! # Algebra
//!
//! This module contains a trait that defines a bidirectional
//! structural algebra with a fold-unfold operation (bubble).

pub mod implementations;

/// Trait for "combining" two things into one thing.
pub trait Semigroup {
    /// Associative function which combines two instances of the semigroup.
    fn combine(self, other: Self) -> Self;
}

/// Trait for a type that has an "empty" or "identity" element.
pub trait Empty {
    fn empty() -> Self;
}

/// A [`Semigroup`] that also has an [`Empty`] element.
pub trait Monoid: Semigroup + Empty {}
impl<T> Monoid for T where T: Semigroup + Empty {}

/// Type for something that can bubble a value down its structure (anamorphism)
/// and bubble up a response (catamorphism).
pub trait Bubble<Down, Up> {
    fn bubble(&mut self, cx: &mut Down) -> Up;
}

/// Like [`Bubble`] but combines the result into a cartesian product
/// (a buffer) instead of using monoids.
pub trait Gather<Context, Item> {
    const SIZE: usize;

    fn gather(
        &mut self,
        cx: &mut Context,
        acc: &mut [std::mem::MaybeUninit<Item>],
    );
}

// #[cfg(feature = "specialization")]
// impl<T, Down, Up> Bubble<Down, Up> for T
// where
//     Up: Empty,
// {
//     default fn bubble(&mut self, cx: &mut Down) -> Up {
//         Up::empty()
//     }
// }
