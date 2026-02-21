#![allow(unused)]

use vek::Vec2;
use {
    ui_composer_core::app::composition::algebra::Bubble,
    ui_composer_input::event::{CursorEvent, Event},
};
use {ui_composer_state::futures_signals::signal::Mutable, vek::Rect};

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

impl Bubble<Event, bool> for Hover {
    fn bubble(&mut self, event: &mut Event) -> bool {
        match event {
            Event::Cursor { id, event } => match event {
                CursorEvent::Moved { position } => {
                    let rect_contains_point =
                        self.rect.contains_point(*position + Vec2::new(1.0, 1.0));
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
