#![allow(unused)]
use super::InputItem;
use crate::app::primitives::{Primitive, Event};
use futures_signals::signal::Mutable;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use vek::Rect;

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
    fn handle_event(&mut self, _event: Event) -> bool {
        // match event {
        //     UIEvent::CursorMoved {
        //         device_id: _,
        //         position,
        //     } => {
        //         let position = Vec2::new(position.x as f32, position.y as f32);
        //         let rect_contains_point = self.rect.contains_point(position);
        //         self.is_hovered_state
        //             .set_if(rect_contains_point, |a, b| a != b);
        //         true
        //     }
        //     UIEvent::CursorLeft { device_id: _ } => {
        //         self.is_hovered_state.set(false);
        //         false
        //     }
        //     _ => false,
        // }
        false
    }

    fn poll_processors(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}
