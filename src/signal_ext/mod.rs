use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_signals::signal::Signal;
use pin_project::pin_project;

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
