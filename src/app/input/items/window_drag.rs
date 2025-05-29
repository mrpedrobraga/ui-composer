#![allow(unused)]
use crate::app::primitives::PollProcessors;
use {
    super::super::{Event, InputItem},
    crate::{
        app::{
            input::{ButtonState, MouseButton},
            primitives::Primitive,
        },
        prelude::CursorEvent,
    },
    core::{
        pin::Pin,
        task::{Context, Poll},
    },
    futures_signals::signal::Mutable,
    vek::Rect,
};

/// An Interactor that handles a user dragging the window.
pub struct Drag {
    rect: Rect<f32, f32>,
    is_hovered_state: Mutable<bool>,
    is_dragging_state: Mutable<bool>,
}

impl Drag {
    pub fn new(
        rect: Rect<f32, f32>,
        is_hovered_state: Mutable<bool>,
        is_dragging_state: Mutable<bool>,
    ) -> Self {
        Self {
            rect,
            is_hovered_state,
            is_dragging_state,
        }
    }
}

impl InputItem for Drag {}

impl Primitive for Drag {
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
                CursorEvent::Button(button, state) if self.is_hovered_state.get() => {
                    match (button, state) {
                        (MouseButton::Left, ButtonState::Pressed) => {
                            self.is_dragging_state.set(true);
                            true
                        }
                        (MouseButton::Left, ButtonState::Pressed) => {
                            self.is_dragging_state.set(false);
                            false
                        }
                        _ => false,
                    }
                }
                _ => false,
            },
            _ => false,
        }
    }
}

impl PollProcessors for Drag {
    fn poll_processors(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}
