#![allow(unused)]
use crate::app::primitives::PollProcessors;
use {
    super::super::{Event, InputItem},
    crate::{
        app::{
            input::{ButtonState, MouseButton},
            primitives::Primitive,
        },
        prelude::{CursorEvent, Effect},
        state::Mutable,
    },
    core::{
        pin::Pin,
        task::{Context, Poll},
    },
    vek::{Rect, Vec2},
};

/// An Interactor that handles a user hovering over it with a cursor.
pub struct Tap<A: Effect> {
    rect: Rect<f32, f32>,
    mouse_position_state: Mutable<Option<Vec2<f32>>>,
    tap_effect: A,
}

impl<Fx> Tap<Fx>
where
    Fx: Effect,
{
    pub fn new(
        rect: Rect<f32, f32>,
        mouse_position_state: Mutable<Option<Vec2<f32>>>,
        tap_effect: Fx,
    ) -> Self {
        Self {
            rect,
            mouse_position_state,
            tap_effect,
        }
    }
}

impl<A> InputItem for Tap<A> where A: Effect + Send {}

impl<A> Primitive for Tap<A>
where
    A: Effect + Send + Sync,
{
    fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::Cursor { id: _, event } => match event {
                CursorEvent::Moved { position } => {
                    self.mouse_position_state.set(Some(position));
                    false
                }
                CursorEvent::Button(MouseButton::Left, ButtonState::Pressed)
                    if self
                        .mouse_position_state
                        .get()
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

impl<A> PollProcessors for Tap<A>
where
    A: Effect + Send + Sync,
{
    fn poll_processors(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}
