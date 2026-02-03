use futures_signals::signal::Signal;
use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::elements::{DummyEnvironment, Element};

/// Has a reference to a runner, serving as an Executor for its [`Future`]s and [`Signal`]s.
#[pin_project(project=AsyncExecutorProj)]
pub struct DummyExecutor<A> {
    #[pin]
    pub element: A,
}

impl<A> Signal for DummyExecutor<A> where A: Element<DummyEnvironment> {
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let AsyncExecutorProj { element } = self.project();
        element.poll(cx, &DummyEnvironment())
    }
}
