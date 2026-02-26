#![allow(unused)]
use ui_composer_core::app::composition::algebra::Bubble;
use ui_composer_core::app::composition::elements::{Blueprint, Element};
use ui_composer_input::event::{ButtonState, MouseButton};
use ui_composer_input::event::{CursorEvent, Event, TouchStage};
use ui_composer_math::glamour::Contains;
use ui_composer_math::prelude::{Point2, Rect};
use ui_composer_state::effect::Effect;
use ui_composer_state::futures_signals::signal::Mutable;

/// An Interactor that handles a user hovering over it with a cursor.
pub struct Tap<A: Effect> {
    pub rect: Rect,
    pub tap_effect: A,
    is_hovered_state: Mutable<bool>,
    /// Todo: this shouldn't be saved per `Tap`,
    /// instead, it should be provided to us in a cascading context.
    mouse_position_state: Option<Point2>,
}

impl<Fx> Tap<Fx>
where
    Fx: Effect,
{
    pub fn new(rect: Rect, tap_effect: Fx) -> Self {
        Self {
            rect,
            mouse_position_state: None,
            tap_effect,
            is_hovered_state: Mutable::new(false),
        }
    }

    pub fn with_hover_state(self, is_hovered_state: Mutable<bool>) -> Self {
        Self {
            is_hovered_state,
            ..self
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
                    self.is_hovered_state.set(self.rect.contains(position));
                    false
                }
                CursorEvent::Exited => {
                    self.is_hovered_state.set(false);
                    false
                }
                CursorEvent::Button(
                    MouseButton::Left,
                    ButtonState::Pressed,
                )
                | CursorEvent::Touched {
                    stage: TouchStage::Started,
                    ..
                } if self.is_hovered_state.get() => {
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
