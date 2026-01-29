//! # Emit
//!
//! A trait for something that can emit primitives onto a buffer, like bytes or triangles.
//! If you find similarities between this trait and allocation, you're not imagining it.
//!
//! If you have a type `A` which emits `A'`, and a type `B` which emits `B'`,
//! a tuple `(A, (A, B))` will emit `[A', A', B']`, effectively a `.flat_map()`;
//!
//! A type `T` can be "statically allocated" to a stack of primitives if it is "Sized", that is,
//! it emits the same amount of primitives every time.

use std::mem::MaybeUninit;
use image::Primitive;

/// The main trait of this module, marks that this type can emit `Self::COUNT` instances of a `Primitive`.
pub trait Emit<Primitive> {
    /// How many `Primitive`s this type emits.
    const COUNT: usize;

    /// Emits `Self::COUNT` primitives to a slice of `Self::COUNT` length.
    fn emit(&self, alloc: &[MaybeUninit<Primitive>]);
}

pub mod implementations {
    use std::mem::MaybeUninit;
    use super::Emit;

    impl<A, B, Primitive> Emit<Primitive> for (A, B) where A: Emit<Primitive>, B: Emit<Primitive> {
        const COUNT: usize = A::COUNT + B::COUNT;

        fn emit(&self, alloc: &[MaybeUninit<Primitive>]) {
            let (left, right) = alloc.split_at(A::COUNT);
            self.0.emit(left);
            self.1.emit(right);
        }
    }
}
