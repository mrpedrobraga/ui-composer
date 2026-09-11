//! # Algebra
//!
//! This module contains a trait that defines a bidirectional
//! structural algebra with a fold-then-unfold operation (bubble).

pub mod implementations;

/// Marks a type where items can be "combined" into a new item in a canonical way.
pub trait Semigroup {
    /// Associative function which combines two items from the semigroup.
    fn combine(self, other: Self) -> Self;
}

/// Marks a type where there's an "empty" element.
pub trait Empty {
    /// Returns the empty element.
    fn empty() -> Self;
}

/// Marks a type that:
/// 1. Has a canonical operation to "combine" two items into one;
/// 2. Has a canonical "empty" element.
/// 
/// Notably, this trait is simply an alias trait for `Semigroup + Empty`.
pub trait Monoid: Semigroup + Empty {}
impl<T> Monoid for T where T: Semigroup + Empty {}

/// Type for something that can bubble a value down its structure (anamorphism)
/// and bubble up a response (catamorphism).
pub trait Bubble<Down, Up> {
    /// Pushes `cx` down the tree, gathering `Up`s on the way back.
    /// 
    /// If called recursively on a "tree" structure where the nodes
    /// have several children, it's expected that Down will be either split or cloned,
    /// and that Up will be [`Semigroup::combine`]d.
    fn bubble(&mut self, cx: &mut Down) -> Up;
}

/// Type for something that can bubble a value down its structure (anamorphism)
/// and gather the responses as a tree, represented flat in a buffer.
/// 
/// Like [`Bubble`] but instead of 
pub trait Gather<Context, Item> {
    const SIZE: usize;

    /// Pushes `cx` down the tree, gathering `Up`s on the way back within `acc`.
    /// 
    /// If called recursively on a "tree" structure where the nodes
    /// have several children, it's expected that Down will be either split or cloned.
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
