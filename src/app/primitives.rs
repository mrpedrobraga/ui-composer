use {
    super::input::Event,
    core::{
        pin::Pin,
        task::{Context, Poll},
    },
};

pub trait Primitive: PollProcessors + Send {
    /// Handles an Event (or not). Returns whether the event was handled.
    fn handle_event(&mut self, event: Event) -> bool;
}

/// A trait for a value that describes a [Primitive].
///
/// This trait exists because [Primitive]s might require references
/// to runtime resources (buffers and stuff) that the user does not
/// have access when building their components.
pub trait PrimitiveDescriptor: Primitive {}
impl<A> PrimitiveDescriptor for A where A: Primitive {}

/// A trait representing a [Primitive] or [Node] that might
/// process a [Future] or [Signal].
pub trait PollProcessors: Send {
    /// Polls this node's processors: `Future`s and `Signal`s.
    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>>;
}

/// A trait for an application Node.
pub trait Node {}

/// A trait for a value that describes a [Node].
///
/// This trait exists because [Node]s might require references
/// to runtime resources (buffers and stuff) that the user does not
/// have access when building their components.
pub trait NodeDescriptor {}
