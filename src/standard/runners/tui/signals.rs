use std::io::stdout;
use super::runner::Own;
use crate::runners::tui::Element;
use futures_signals::signal::Signal;
use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use vek::Rect;

/// Has a reference to a runner, serving as an Executor for its [`Future`]s and [`Signal`]s.
#[pin_project(project=AsyncExecutorProj)]
pub struct AsyncExecutor<A: Element> {
    #[pin]
    element: Own<A>,
}

impl<A: Element> AsyncExecutor<A> {
    pub fn new(element: Own<A>) -> Self {
        AsyncExecutor { element }
    }
}

impl<A: Element> Signal for AsyncExecutor<A> {
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let AsyncExecutorProj { element } = self.project();

        if let Ok(mut element_borrow) = element.try_borrow_mut() {
            let pinned_element = unsafe { Pin::new_unchecked(&mut *element_borrow) };
            pinned_element.poll(cx, &mut ())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
