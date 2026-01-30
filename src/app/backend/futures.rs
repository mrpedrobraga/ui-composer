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

use std::ops::DerefMut;
use std::pin::Pin;
use std::task::{Context, Poll};
use pin_project::pin_project;
use futures_signals::signal::Signal;
use crate::app::backend::{AppContext, Runner};

#[cfg(feature = "std")]
type Own<A> = std::sync::Arc<std::sync::Mutex<A>>;
#[cfg(not(feature = "std"))]
type Own<A> = spin::Mutex<A>;

/// Has a reference to a runner, serving as an Executor for its [`Future`]s and [`Signal`]s.
#[pin_project(project=AsyncExecutorProj)]
pub struct AsyncExecutor<A: Runner> {
    #[pin]
    runner: Own<A>,
}

impl<A: Runner> AsyncExecutor<A> {
    pub fn new(runner: Own<A>) -> Self {
        AsyncExecutor { runner }
    }
}

impl<A: Runner> Signal for AsyncExecutor<A> {
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let AsyncExecutorProj { runner } = self.project();

        let mut runner = runner.lock().unwrap();
        let runner = runner.deref_mut();
        let runner = unsafe { Pin::new_unchecked(runner) };

        runner.process(cx, &mut AppContext {})
    }
}