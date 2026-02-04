use super::runner::Own;
use crate::app::composition::elements::Element;
use crate::runners::tui::runner::TUIEnvironment;
use futures_signals::signal::Signal;
use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Has a reference to a runner, serving as an Executor for its [`Future`]s and [`Signal`]s.
#[pin_project(project=AsyncExecutorProj)]
pub struct AsyncExecutor<App: Element<TUIEnvironment>> {
    #[pin]
    element: Own<App>,
}

impl<App: Element<TUIEnvironment>> AsyncExecutor<App> {
    pub fn new(element: Own<App>) -> Self {
        AsyncExecutor { element }
    }
}

impl<App: Element<TUIEnvironment>> Signal for AsyncExecutor<App> {
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let AsyncExecutorProj { element } = self.project();

        if let Ok(mut element_borrow) = element.try_borrow_mut() {
            let pinned_element = unsafe { Pin::new_unchecked(&mut *element_borrow) };
            // TODO: Change how this environment is sourced!
            pinned_element.poll(cx, &TUIEnvironment)
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
