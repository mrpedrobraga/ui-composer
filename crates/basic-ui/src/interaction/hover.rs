#![allow(unused)]

use {
    ui_composer_core::app::composition::algebra::Bubble,
    ui_composer_input::event::{CursorEvent, Event},
    ui_composer_math::glamour::Contains,
};
use {
    ui_composer_math::prelude::Rect,
    ui_composer_state::futures_signals::signal::Mutable,
};

/// An Interactor that handles a user hovering over it with a cursor.
pub struct Hover {
    rect: Rect,
    is_hovered_state: Mutable<bool>,
}

impl Hover {
    pub fn new(rect: Rect, is_hovered_state: Mutable<bool>) -> Self {
        Self {
            rect,
            is_hovered_state,
        }
    }
}

impl Bubble<Event, bool> for Hover {
    fn bubble(&mut self, event: &mut Event) -> bool {
        match event {
            Event::Cursor { id, event } => match event {
                CursorEvent::Moved { position } => {
                    let rect_contains_point = self.rect.contains(position);
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
}
