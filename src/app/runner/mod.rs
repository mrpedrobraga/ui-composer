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

pub mod futures;

/// An application runner. It bubbles down Affects and bubble up Effects,
/// effectively allowing the pure, immutable application to perform IO.
pub trait Runner {
    type AppBlueprint;

    /// The entry point of the runner, responsible for setup.
    /// `UIComposer` will call this method when you call `UIComposer::run_custom<_>`.
    fn run(ui: Self::AppBlueprint);
}

/// Additional context for processing an app in a Runner.
pub struct AppContext {}
