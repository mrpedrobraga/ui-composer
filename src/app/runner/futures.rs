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
pub struct AsyncExecutor<Env, App: Element<Env>> {
    #[pin]
    element: Own<App>,
    environment: Env,
    initial_yield: bool,
}

impl<Env, App: Element<Env>> AsyncExecutor<Env, App> {
    pub fn new(element: Own<App>, environment: Env) -> Self {
        AsyncExecutor {
            element,
            environment,
            initial_yield: true,
        }
    }
}

impl<Env, App: Element<Env>> Signal for AsyncExecutor<Env, App> {
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let AsyncExecutorProj {
            element,
            environment,
            initial_yield,
        } = self.project();

        if let Ok(mut element_borrow) = element.lock() {
            let pinned_element = unsafe { Pin::new_unchecked(&mut *element_borrow) };
            
            // Because of how signals work, we must yield at least once!
            if *initial_yield {
                *initial_yield = false;
                Poll::Ready(Some(()))
            } else {
                pinned_element.poll(cx, environment)
            }
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
