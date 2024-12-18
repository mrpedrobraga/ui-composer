use futures_signals::signal::Signal;
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub use futures_signals::signal::Mutable as Editable;
pub use futures_signals::signal::SignalExt;
pub use futures_signals::signal_map::MutableBTreeMap as EditableMap;
pub use futures_signals::signal_map::SignalMapExt;
pub use futures_signals::signal_vec::MutableVec as EditableVec;
pub use futures_signals::signal_vec::SignalVecExt;

/// An attribute is a value that might be unset, directly set or reactive.
#[pin_project(project = AttributeProj)]
pub enum Attribute<T, S = ()> {
    Fixed(T),
    Reactive(#[pin] S),
}

impl<T, S: Signal<Item = T>> Signal for Attribute<T, S> {
    type Item = T;

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let me = self.project();

        match me {
            // A fixed Attribute never updates itself, and therefore exhausts itself immediately.
            AttributeProj::Fixed(_) => Poll::Ready(None),
            // A reactive Attribute is a pass-through.
            AttributeProj::Reactive(signal) => signal.poll_change(cx),
        }
    }
}

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
