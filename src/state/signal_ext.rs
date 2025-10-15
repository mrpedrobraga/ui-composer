use crate::state::process::SignalReactItem;
use core::task::Poll;
use futures_signals::signal::{Map, SignalExt};
use futures_signals::signal::{Mutable, MutableSignal};

/// Combines two Polls into a single poll describing a task with two parts.
/// The result is decided based on a precedence order.
///
/// If any of the values are Poll::Ready(Some(())), the result will be Poll::Ready(Some(()));
/// If not and any of the values are Poll::Pending, the result will be Poll::Pending;
/// Otherwise (if both values are Poll::Ready(None)), the result will be Poll::Ready(None);
pub fn coalesce_polls(poll_a: Poll<Option<()>>, poll_b: Poll<Option<()>>) -> Poll<Option<()>> {
    match (poll_a, poll_b) {
        (Poll::Ready(None), Poll::Ready(None)) => Poll::Ready(None),
        (Poll::Pending, Poll::Pending) => Poll::Pending,
        (Poll::Ready(None), Poll::Pending) => Poll::Pending,
        (Poll::Pending, Poll::Ready(None)) => Poll::Pending,
        (Poll::Ready(Some(())), Poll::Ready(Some(()))) => Poll::Ready(Some(())),
        (Poll::Ready(None), Poll::Ready(Some(()))) => Poll::Ready(Some(())),
        (Poll::Ready(Some(())), Poll::Ready(None)) => Poll::Ready(Some(())),
        (Poll::Ready(Some(())), Poll::Pending) => Poll::Ready(Some(())),
        (Poll::Pending, Poll::Ready(Some(()))) => Poll::Ready(Some(())),
    }
}

pub trait MutableExt {
    type Item;
    type Output<F, U>
    where
        F: FnMut(Self::Item) -> U,
        Self::Item: Copy;

    fn derive<F, U>(&self, predicate: F) -> Self::Output<F, U>
    where
        F: FnMut(Self::Item) -> U,
        Self::Item: Copy;
}

impl<Item> MutableExt for Mutable<Item> {
    type Item = Item;
    type Output<F, U>
        = SignalReactItem<Map<MutableSignal<Item>, F>>
    where
        F: FnMut(Self::Item) -> U,
        Self::Item: Copy;

    fn derive<F, U>(&self, predicate: F) -> Self::Output<F, U>
    where
        F: FnMut(Self::Item) -> U,
        Self::Item: Copy,
    {
        SignalReactItem(self.signal().map(predicate))
    }
}
