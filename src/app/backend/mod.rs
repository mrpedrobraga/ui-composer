//! # Backend
//!
//! This module declares the [`Runner`] trait, which is the procedural "runtime" that can
//! interpret and execute an application defined functionally-declaratively.
//!
//! As seen in [`crate::app`], the UI is defined in terms of composable building blocks,
//! all of which are functions in the FP sense: pure, referentially transparent, etc.
//!
//! Whereas that does mean they can not do any side effects (such as rendering to the screen),
//! they can _define_ effectful actions. These will bubble up from the leaves of the UI tree
//! all the way to the root, which is a [`Runner`]. Then, effects will be executed.

use core::{
    ops::DerefMut as _,
    pin::Pin,
    task::{Context, Poll},
};
use futures_signals::signal::Signal;
use pin_project::pin_project;

/// The layer of the application that stands between the app and the outside world.
pub trait Runner {
    /// The type of the Node tree this Backend executes.
    type UI;

    /// Blocking function that runs the application.
    fn run(ui: Self::UI);

    /// Polls the `Futures` and `Signals` from the node tree.
    #[allow(unused_variables)]
    fn process(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &mut AppContext,
    ) -> Poll<Option<()>> {
        Poll::Ready(None)
    }
}

#[cfg(feature = "std")]
type Own<A> = std::sync::Arc<std::sync::Mutex<A>>;
#[cfg(not(feature = "std"))]
type Own<A> = spin::Mutex<A>;

/// A futures-based construct that polls the engine's processes.
#[pin_project(project=EffectExecutorProj)]
pub struct EffectExecutor<A: Runner> {
    #[pin]
    runner: Own<A>,
}

impl<A: Runner> EffectExecutor<A> {
    pub fn new(runner: Own<A>) -> Self {
        EffectExecutor { runner }
    }
}

impl<A: Runner> Signal for EffectExecutor<A> {
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let EffectExecutorProj { runner } = self.project();

        let mut runner = runner.lock().unwrap();
        let runner = runner.deref_mut();
        let runner = unsafe { Pin::new_unchecked(runner) };

        runner.process(cx, &mut AppContext {})
    }
}

pub struct AppContext {}
