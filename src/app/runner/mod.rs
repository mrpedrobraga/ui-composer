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
use ::futures::Stream;
use crate::app::input::Event;

pub mod futures;

/// An application runner. It bubbles down Affects and bubble up Effects,
/// effectively allowing the pure, immutable application to perform IO.
pub trait Runner {
    type AppBlueprint;

    /// The entry point of the runner, responsible for setup.
    /// `UIComposer` will call this method when you call `UIComposer::run_custom<_>`.
    fn run(ui: Self::AppBlueprint) -> Self;

    /// `UIComposer` will call this method once to generate a stream of events the app
    /// will receive and react to.
    fn event_stream(&mut self) -> impl Stream<Item = Event> + Send + Sync + 'static;

    /// `UIComposer` will call this method whenever the app reacts to anything.
    ///
    /// This might be a reaction to an event (like window resizing or a click),
    /// or a `Future` or a `Signal` that resolved somewhere in your app.
    fn on_update(&mut self);

    /// `UIComposer` will call this function on the main thread after setting up the execution
    /// of events and reactivity.
    ///
    /// This is necessary for platforms that need anything to be done on the main thread specifically.
    fn main_loop(&mut self) {}

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
