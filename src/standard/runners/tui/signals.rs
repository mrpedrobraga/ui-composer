use super::runner::Own;
use crate::app::composition::elements::Element;
use crate::runners::tui::runner::TerminalEnvironment;
use futures_signals::signal::Signal;
use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Has a reference to a runner, serving as an Executor for its [`Future`]s and [`Signal`]s.
#[pin_project(project=AsyncExecutorProj)]
pub struct AsyncExecutor<App: Element<TerminalEnvironment>> {
    #[pin]
    element: Own<App>,
}

impl<App: Element<TerminalEnvironment>> AsyncExecutor<App> {
    pub fn new(element: Own<App>) -> Self {
        AsyncExecutor { element }
    }
}

impl<App: Element<TerminalEnvironment>> Signal for AsyncExecutor<App> {
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let AsyncExecutorProj { element } = self.project();

        if let Ok(mut element_borrow) = element.lock() {
            let pinned_element = unsafe { Pin::new_unchecked(&mut *element_borrow) };
            // TODO: Change how this environment is sourced!
            pinned_element.poll(cx, &TerminalEnvironment)
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
