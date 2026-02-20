#![allow(unused)]
use crate::app::composition::algebra::Bubble;
use crate::app::composition::elements::{Blueprint, Element};
use crate::app::input::CursorEvent;
use crate::prelude::TouchStage;
use crate::runners::tui::runner::TerminalEnvironment;
use crate::state::effect::Effect;
use {
    super::super::{Event, InputItem},
    crate::{
        app::input::{ButtonState, MouseButton},
        state::Mutable,
    },
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
    pub fn new(
        rect: Rect<f32, f32>,
        tap_effect: Fx,
    ) -> Self {
        Self {
            rect,
            mouse_position_state: None,
            tap_effect,
        }
    }
}

impl<A> InputItem for Tap<A> where A: Effect + Send {}

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
                CursorEvent::Button(
                    MouseButton::Left,
                    ButtonState::Pressed,
                )
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

impl<A> Blueprint<TerminalEnvironment> for Tap<A>
where
    A: Effect + Send + Sync + 'static,
{
    type Element = Self;

    fn make(self, env: &TerminalEnvironment) -> Self::Element {
        self
    }
}

impl<A> Element<TerminalEnvironment> for Tap<A>
where
    A: Effect + Send + Sync + 'static,
{
    type Effect<'fx> = ();

    fn effect(&self) -> Self::Effect<'_> {}
}
