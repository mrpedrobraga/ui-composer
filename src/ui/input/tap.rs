#![allow(unused)]
use super::InputItem;
use crate::app::node::{AppItem, UIEvent};
use crate::prelude::Effect;
use crate::state::Mutable;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use vek::{Rect, Vec2};

/// An Interactor that handles a user hovering over it with a cursor.
pub struct Tap<A: Effect> {
    rect: Rect<f32, f32>,
    mouse_position_state: Mutable<Option<Vec2<f32>>>,
    tap_action: A,
}

impl<Fx> Tap<Fx>
where
    Fx: Effect,
{
    pub fn new(
        rect: Rect<f32, f32>,
        mouse_position_state: Mutable<Option<Vec2<f32>>>,
        tap_state: Fx,
    ) -> Self {
        Self {
            rect,
            mouse_position_state,
            tap_action: tap_state,
        }
    }
}

impl<A> InputItem for Tap<A> where A: Effect + Send {}

impl<A> AppItem for Tap<A>
where
    A: Effect + Send + Sync,
{
    fn handle_ui_event(&mut self, _event: UIEvent) -> bool {
        // match event {
        //     UIEvent::CursorMoved {
        //         device_id: _,
        //         position,
        //     } => {
        //         self.mouse_position_state
        //             .set(Some(Vec2::new(position.x, position.y).as_()));
        //         false
        //     }
        //     UIEvent::MouseInput {
        //         device_id: _,
        //         state,
        //         button,
        //     } => match (button, state) {
        //         (winit::event::MouseButton::Left, winit::event::ElementState::Pressed) => {
        //             if let Some(mouse_position) = self.mouse_position_state.get() {
        //                 if self.rect.contains_point(mouse_position) {
        //                     self.tap_action.apply();
        //                     true
        //                 } else {
        //                     false
        //                 }
        //             } else {
        //                 false
        //             }
        //         }
        //         _ => false,
        //     },
        //     _ => false,
        // }
        false
    }

    fn poll_processors(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}
