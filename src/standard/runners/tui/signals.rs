use futures_signals::signal::Signal;
use pin_project::pin_project;
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use crate::runners::tui::Element;

pub type Own<A> = Arc<Mutex<A>>;

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

        let mut e = element.lock().unwrap();
        let e = e.deref_mut();
        let e = unsafe { Pin::new_unchecked(e) };

        e.poll(cx, &mut ())
    }
}
