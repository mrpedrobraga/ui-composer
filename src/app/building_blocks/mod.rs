//! # Composition Primitives
//!
//! UI Composer is a library built around the concept of reactive composition.
//! A [BuildingBlock] is a quantum of functionality, which can be composed with other [BuildingBlock]s
//! to create complex functionality.
//!
//! The "functionality tree" builds [Reifiable]s with Algebraic Data Types.
//! Because Rust uses Zero-Cost abstractions for ADTs, this means that the "tree'
//! doesn't *really* exist in compile time.
//!
//! Primitives are rarely created directly, instead, they defined by [Reifiable]s.
//!
//! ```compile_fail
//! use ui_composer::prelude::*;
//!
//! // Composing app nodes to create apps.
//! Window(())
//! // Composing layout items to create reactive layouts.
//! Center(WithSize(..., Button(...))
//! // Composing graphic/text/input primitives to create standard.
//! items!(Hover(...), Graphic(...), Text(...))
//! ```
//!
//! The descriptors are composed together, usually with functions,
//! then they are _Output_ into Primitives.
//!
//! [Signal]s themselves are primitives and have the ability
//! to replace parts of the "functionality tree" on the fly.
//!
//! This functionality is what powers, for example, [LayoutItem]'s ability to
//! re-render its items on demand.

use crate::state::process::Pollable;
use super::input::Event;

pub mod emit;
pub mod reify;
pub mod implementations;

pub trait BuildingBlock<Resources>: Pollable<Resources> + Send {
    /// Handles an Event (or not). Returns whether the event was handled.
    fn handle_event(&mut self, event: Event) -> bool;
}
