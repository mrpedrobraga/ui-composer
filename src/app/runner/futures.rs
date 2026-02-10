//! # Async
//!
//! An application is defined at compile-time, statically. But your program might contain values which
//! are made available only at run-time ([`Future`] or IO) or that repeatedly change along
//! the duration of the program ([`Signal`]).
//!
//! In either case, the UI might want to "React" to it.
//!
//! In this crate, the description of how reactive UI reacts to signal changes is done with
//! code from [`crate::state`]. But, like `Future`s need executors, this crate offers "App executors",
//! which can poll the app's futures and signals.

use crate::app::composition::elements::Element;
use futures_signals::signal::Signal;
use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(feature = "std")]
type Own<A> = std::sync::Arc<std::sync::Mutex<A>>;
#[cfg(not(feature = "std"))]
type Own<A> = spin::Mutex<A>;

/// Has a reference to a runner, serving as an Executor for its [`Future`]s and [`Signal`]s.
#[pin_project(project=AsyncExecutorProj)]
pub struct AsyncExecutor<Env, App: Element<Env>, Callback> {
    #[pin]
    element: Own<App>,
    environment: Env,
    first_tick: bool,
    callback: Callback,
}

impl<Env, App: Element<Env>, Callback> AsyncExecutor<Env, App, Callback> {
    pub fn new(
        element: Own<App>,
        environment: Env,
        callback: Callback,
    ) -> Self {
        AsyncExecutor {
            element,
            environment,
            first_tick: true,
            callback,
        }
    }
}

impl<Env, App: Element<Env>, Callback: FnMut()> Signal
    for AsyncExecutor<Env, App, Callback>
{
    type Item = ();

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Self::Item>> {
        let AsyncExecutorProj {
            element,
            environment,
            first_tick,
            callback,
        } = self.project();

        if let Ok(mut element_borrow) = element.lock() {
            let pinned_element =
                unsafe { Pin::new_unchecked(&mut *element_borrow) };

            // Because of how signals work internally, we must yield at least once.
            let inner_poll = pinned_element.poll(cx, environment);
            if let Poll::Ready(None) = inner_poll
                && *first_tick
            {
                *first_tick = false;
                (callback)();

                return Poll::Ready(Some(()));
            }
            inner_poll
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
