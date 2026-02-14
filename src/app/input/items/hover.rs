#![allow(unused)]

use vek::Vec2;
use crate::app::composition::algebra::Bubble;
use crate::app::composition::elements::{Blueprint, Element};
use crate::app::input::CursorEvent;
use crate::runners::tui::runner::TerminalEnvironment;
use {
    super::super::{Event, InputItem},
    futures_signals::signal::Mutable,
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

impl Blueprint<TerminalEnvironment> for Hover {
    type Element = Self;

    fn make(self, env: &TerminalEnvironment) -> Self::Element {
        self
    }
}

impl Element<TerminalEnvironment> for Hover {
    type Effect<'fx> = ();

    fn effect(&self) -> Self::Effect<'_> {}
}
