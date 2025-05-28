#![allow(unused)]
use {
    super::InputItem,
    crate::app::primitives::{Primitive, Event},
    futures_signals::signal::Mutable,
    std::{
        pin::Pin,
        task::{Context, Poll},
    },
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
        //     UIEvent::MouseInput {
        //         device_id: _,
        //         state,
        //         button,
        //     } => match (button, state) {
        //         (winit::event::MouseButton::Left, winit::event::ElementState::Pressed) => {
        //             if self.is_hovered_state.get() {
        //                 self.is_dragging_state.set(true);
        //                 true
        //             } else {
        //                 false
        //             }
        //         }
        //         (winit::event::MouseButton::Left, winit::event::ElementState::Released) => {
        //             self.is_dragging_state.set(false);
        //             false
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
