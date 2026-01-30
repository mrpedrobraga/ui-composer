//! # Gather
//!
//! A trait for something that can emit primitives onto an accumulator, like a buffer of bytes or triangles.
//! This is basically the trait for `Catamorphism` (fold).
//!
//! If you have a type `A` which emits `A'`, and a type `B` which emits `B'`,
//! a tuple `(A, (A, B))` will emit `[A', A', B']`, effectively a `.fold()`;
//!
//! A type `T` can be "statically allocated" to a stack of primitives if it is "Sized", that is,
//! it emits the same amount of primitives every time.

use std::mem::MaybeUninit;

/// The main trait of this module, marks that this type can emit `Self::COUNT` instances of a `Primitive`.
pub trait Gather<Primitive> {
    /// How many `Primitive`s this type emits.
    const COUNT: usize;

    /// Emits `Self::COUNT` primitives to a slice of `Self::COUNT` length.
    fn emit(&self, alloc: &[MaybeUninit<Primitive>]);
}

pub mod implementations {
    use std::mem::MaybeUninit;
    use super::Gather;

    impl<A, B, Primitive> Gather<Primitive> for (A, B) where A: Gather<Primitive>, B: Gather<Primitive> {
        const COUNT: usize = A::COUNT + B::COUNT;

        fn emit(&self, alloc: &[MaybeUninit<Primitive>]) {
            let (left, right) = alloc.split_at(A::COUNT);
            self.0.emit(left);
            self.1.emit(right);
        }
    }
}
