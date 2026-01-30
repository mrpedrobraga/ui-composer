//! # Reify
//!
//! [`Reify`]s are structures which describe other structures. They are useful for describing
//! applications in terms of runtime resources you have yet to acquire, like GPU buffers.
//!
//! They do something similar to `Reader` monads or closure, allowing you to capture a "Context"
//! and specify something in terms of it while you don't yet have it.
//!
//! `reify` is a homomorphism (effectively `Functor::map`).

pub mod implementations;

/// The main trait of this module.
pub trait Reify<Context> {
    type Output;
    fn reify(self, context: &mut Context) -> Self::Output;
}
