use std::{
    pin::Pin,
    task::{Context, Poll},
};
use vek::Vec2;

//MARK: App Items

pub trait Primitive: Send {
    /// Handles an Event (or not). Returns whether the event was handled.
    fn handle_event(&mut self, event: Event) -> bool;

    /// Polls this node's processors: `Future`s and `Signal`s.
    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>>;
}

pub trait PrimitiveDescriptor: Primitive {}
impl<A> PrimitiveDescriptor for A where A: Primitive {}

//MARK: Events

pub type Num = f32;

#[derive(Default, Clone, PartialEq)]
pub enum Event {
    #[default]
    None,
    Cursor {
        id: u32,
        event: CursorEvent,
    },
}

#[derive(Clone, PartialEq)]
pub enum CursorEvent {
    Moved { to: Vec2<Num> },
    Clicked {},
}
