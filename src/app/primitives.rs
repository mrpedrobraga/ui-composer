//! # Composition Primitives
//!
//! UI Composer is a library built around the concept of reactive composition.
//! A [Primitive] is a quantum of functionality, which can be composed with other [Primitive]s
//! to create complex functionality.
//!
//! The "functionality tree" builds [Primitives]s with Algebraic Data Types.
//! Because Rust uses Zero-Cost abstractions for ADTs, this means that the "tree'
//! doesn't *really* exist in compile time.
//!
//! Primitives are rarely created directly, instead, they defined by [PrimitiveDescriptor]s.
//!
//! ```compile_fail
//! use ui_composer::prelude::*;
//!
//! // Composing app nodes to create apps.
//! Window(())
//! // Composing layout items to create reactive layouts.
//! Center(WithSize(..., Button(...))
//! // Composing graphic/text/input primitives to create components.
//! items!(Hover(...), Graphic(...), Text(...))
//! ```
//!
//! The descriptors are composed together, usually with functions,
//! then they are _reified_ into Primitives.
//!
//! [Signal]s themselves are primitives and have the ability
//! to replace parts of the "functionality tree" on the fly.
//!
//! This functionality is what powers, for example, [LayoutItem]'s ability to
//! re-render its items on demand.

#[allow(unused)]
use {super::super::layout::LayoutItem, futures_signals::signal::Signal};
use {
    super::input::Event,
    core::{
        pin::Pin,
        task::{Context, Poll},
    },
};

pub trait Primitive<Resources>: Processor<Resources> + Send {
    /// Handles an Event (or not). Returns whether the event was handled.
    fn handle_event(&mut self, event: Event) -> bool;
}

/// A trait for a value that describes a [Primitive].
///
/// This trait exists because [Primitive]s might require references
/// to runtime resources (buffers and stuff) that the user does not
/// have access when building their components.
pub trait PrimitiveDescriptor<Resources> {
    type Primitive: Primitive<Resources>;

    /// Yields the [Primitive] this descriptor describes.
    fn reify(self, resources: &mut Resources) -> Self::Primitive;
}

/// A trait representing a [Primitive] or [Node] that _might_
/// process internal [Future]s or [Signal]s.
#[must_use = "processors are lazy and do nothing unless polled"]
pub trait Processor<Resources>: Send {
    /// Recursively polls this primitive's inner processes (`Future`s and `Signal`s).
    fn poll(
        self: Pin<&mut Self>,
        #[expect(unused)] cx: &mut Context,
        #[expect(unused)] resources: &mut Resources,
    ) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}
