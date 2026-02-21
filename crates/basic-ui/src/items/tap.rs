#![allow(unused)]
use ui_composer_core::app::composition::algebra::Bubble;
use ui_composer_core::app::composition::elements::{Blueprint, Element};
use ui_composer_input::event::{CursorEvent, Event, TouchStage};
use ui_composer_state::effect::Effect;
use {
    ui_composer_input::event::{ButtonState, MouseButton},
    vek::{Rect, Vec2},
};

/// An Interactor that handles a user hovering over it with a cursor.
pub struct Tap<A: Effect> {
    pub rect: Rect<f32, f32>,
    pub tap_effect: A,
    mouse_position_state: Option<Vec2<f32>>,
}

impl<Fx> Tap<Fx>
where
    Fx: Effect,
{
    pub fn new(rect: Rect<f32, f32>, tap_effect: Fx) -> Self {
        Self {
            rect,
            mouse_position_state: None,
            tap_effect,
        }
    }
}

impl<A> Bubble<Event, bool> for Tap<A>
where
    A: Effect + Send + Sync,
{
    fn bubble(&mut self, event: &mut Event) -> bool {
        match event {
            Event::Cursor { id: _, event } => match event {
                CursorEvent::Moved { position } => {
                    self.mouse_position_state = Some(*position);
                    false
                }
                CursorEvent::Button(MouseButton::Left, ButtonState::Pressed)
                | CursorEvent::Touched {
                    stage: TouchStage::Started,
                    ..
                } if self
                    .mouse_position_state
                    .is_some_and(|pos| self.rect.contains_point(pos)) =>
                {
                    self.tap_effect.apply();
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
}

/*

*/
