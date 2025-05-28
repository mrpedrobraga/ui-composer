#![allow(unused)]
use {
    super::super::{Event, InputItem},
    crate::{app::primitives::Primitive, prelude::CursorEvent},
    futures_signals::signal::Mutable,
    std::{
        pin::Pin,
        task::{Context, Poll},
    },
    vek::Rect,
};

/// An Interactor that handles a user hovering over it with a cursor.
pub struct Hover {
    rect: Rect<f32, f32>,
    is_hovered_state: Mutable<bool>,
}

impl Hover {
    pub fn new(rect: Rect<f32, f32>, is_hovered_state: Mutable<bool>) -> Self {
        Self {
            rect,
            is_hovered_state,
        }
    }
}

impl InputItem for Hover {}

impl Primitive for Hover {
    fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::Cursor { id, event } => match event {
                CursorEvent::Moved { position } => {
                    let rect_contains_point = self.rect.contains_point(position);
                    self.is_hovered_state
                        .set_if(rect_contains_point, |a, b| a != b);
                    true
                }
                CursorEvent::Exited => {
                    self.is_hovered_state.set(false);
                    false
                }
                _ => false,
            },
            _ => false,
        }
    }

    fn poll_processors(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}
