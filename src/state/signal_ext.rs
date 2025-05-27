use crate::prelude::Slot;
use futures_signals::signal::Mutable;
use std::task::Poll;

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

