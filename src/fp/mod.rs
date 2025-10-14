//! Trait for a compile-time descriptor of some sort
//! that can be transformed into a type that holds
//! runtime resources.
//!
//! This crate uses this pattern so much, and so here's the trait that embodies it.

/// The fellow in question.
///
/// The trait has a generic (`'re`) that represents the lifetime of the output.
/// This is so the output can depend on references that might exist inside the Context.
pub trait Reifiable<'re, Context> {
    type Output;

    /// This function takes a 'Context'
    /// such that the output type can have access
    /// to run time things.
    fn reify(self, cx: Context) -> Self::Output;
}
