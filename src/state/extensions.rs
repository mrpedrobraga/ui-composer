//! # SignalExt
//!
//! Extension traits and impls to make dealing with states more ergonomic.

impl crate::app::composition::algebra::Semigroup
    for std::task::Poll<Option<()>>
{
    /// If any of the values are Poll::Ready(Some(())), the result will be Poll::Ready(Some(()));
    /// If not and any of the values are Poll::Pending, the result will be Poll::Pending;
    /// Otherwise (if both values are Poll::Ready(None)), the result will be Poll::Ready(None);
    fn combine(self, other: Self) -> Self {
        use std::task::Poll::*;

        match (self, other) {
            (Ready(Some(())), _) | (_, Ready(Some(()))) => Ready(Some(())),
            (Pending, _) | (_, Pending) => Pending,
            _ => Ready(None),
        }
    }
}

impl crate::app::composition::algebra::Empty for std::task::Poll<Option<()>> {
    fn empty() -> Self {
        std::task::Poll::Ready(None)
    }
}
