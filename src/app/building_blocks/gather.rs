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

/// The main trait of this module, marks that this type can emit `Self::COUNT` instances of a `Primitive`.
pub trait Gather {
    type Item;
    type Iter: Iterator<Item = Self::Item>;
    const COUNT: usize;
    fn gather(&self) -> Self::Iter;
}

pub mod implementations {
    use super::{Gather};

    impl<A, B> Gather for (A, B)
    where
        A: Gather,
        B: Gather<Item = A::Item>,
    {
        type Item = A::Item;
        type Iter = std::iter::Chain<A::Iter, B::Iter>;

        const COUNT: usize = A::COUNT + B::COUNT;

        fn gather(&self) -> Self::Iter {
            self.0.gather().chain(self.1.gather())
        }
    }
}
