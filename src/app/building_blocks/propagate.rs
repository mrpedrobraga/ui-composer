//! # Propagate
//!
//! A trait for something that can broadcast affects down its structure.
//! This is basically the trait for `Anamorphism` (unfold).
//!
//! If you have a tuple `(A, (A, B))` and an event `E` is emitted,
//! this trait will take care of passing it down.

/// The main trait of this module, marks a type as being able
/// to broadcast things down.
pub trait Propagate<Affect> {
    fn propagate(&mut self, affect: &mut Affect);
}

pub mod implementations {
    use super::Propagate;

    impl<A, B, Ctx> Propagate<Ctx> for (A, B)
    where
        A: Propagate<Ctx>,
        B: Propagate<Ctx>,
    {
        fn propagate(&mut self, ctx: &mut Ctx) {
            self.0.propagate(ctx);
            self.1.propagate(ctx);
        }
    }
}
