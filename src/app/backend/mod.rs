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
    pin::Pin,
    task::{Context, Poll},
};
pub mod futures;

/// An application runner. It bubbles down Affects and bubble up Effects,
/// effectively allowing the pure, immutable application to perform IO.
pub trait Runner {
    type App;

    /// Runs the application on the main thread.
    fn run(ui: Self::App);

    #[allow(async_fn_in_trait)]
    async fn event_loop(&self);

    #[allow(async_fn_in_trait)]
    async fn react_loop(&self);

    /// Polls the UI as a [`Signal`].
    #[allow(unused_variables)]
    #[deprecated(note = "The runner itself spawns an executor and, thus, shouldn't itself be a Signal.")]
    fn process(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &mut AppContext,
    ) -> Poll<Option<()>> {
        Poll::Ready(None)
    }
}

/// Additional context for processing an app in a Runner.
pub struct AppContext {}
