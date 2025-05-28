use {
    super::input::Event,
    std::{
        pin::Pin,
        task::{Context, Poll},
    },
};

pub trait Primitive: Send {
    /// Handles an Event (or not). Returns whether the event was handled.
    fn handle_event(&mut self, event: Event) -> bool;

    /// Polls this node's processors: `Future`s and `Signal`s.
    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>>;
}

pub trait PrimitiveDescriptor: Primitive {}
impl<A> PrimitiveDescriptor for A where A: Primitive {}
