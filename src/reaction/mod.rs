use futures_signals::signal::Signal;
use pin_project::pin_project;
use std::ops::Range;

use crate::standard::primitive::Primitive;

/// Object that monitors states and reacts by issuing memory update commands.
pub trait Reactor {}

#[pin_project(project = PrimitiveSpliceReactorProj)]
#[must_use = "Signals do nothing unless polled"]
pub struct PrimitiveSpliceReactor<S, I>
where
    S: Signal<Item = I>,
    I: Iterator<Item = Primitive>,
{
    #[pin]
    signal: S,
    range: Range<usize>,
}

impl<S, I> PrimitiveSpliceReactor<S, I>
where
    S: Signal<Item = I>,
    I: Iterator<Item = Primitive>,
{
    pub fn new(signal: S, range: Range<usize>) -> Self {
        Self { signal, range }
    }
}

impl<S, I> Signal for PrimitiveSpliceReactor<S, I>
where
    S: Signal<Item = I>,
    I: Iterator<Item = Primitive>,
{
    type Item = I;

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        let PrimitiveSpliceReactorProj { signal, range: _ } = self.project();
        signal.poll_change(cx)
    }
}
