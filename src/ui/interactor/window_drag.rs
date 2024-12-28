use super::Interactor;
use crate::ui::node::{ItemDescriptor, UIEvent, UIItem};
use futures_signals::signal::Mutable;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use vek::{Rect, Vec2};

/// An Interactor that handles a user dragging the window.
pub struct WindowDrag {
    rect: Rect<f32, f32>,
    is_hovered_state: Mutable<bool>,
    is_dragging_state: Mutable<bool>,
}

impl WindowDrag {
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

impl Interactor for WindowDrag {}
impl ItemDescriptor for WindowDrag {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None // Some(self.area))
    }
}
impl UIItem for WindowDrag {
    fn handle_ui_event(&mut self, event: crate::ui::node::UIEvent) -> bool {
        match event {
            UIEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                let position = Vec2::new(position.x as f32, position.y as f32);
                let rect_contains_point = self.rect.contains_point(position);
                self.is_hovered_state
                    .set_if(rect_contains_point, |a, b| a != b);
                true
            }
            UIEvent::CursorLeft { device_id: _ } => {
                self.is_hovered_state.set(false);
                false
            }
            UIEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => match (button, state) {
                (winit::event::MouseButton::Left, winit::event::ElementState::Pressed) => {
                    if self.is_hovered_state.get() {
                        self.is_dragging_state.set(true);
                        true
                    } else {
                        false
                    }
                }
                (winit::event::MouseButton::Left, winit::event::ElementState::Released) => {
                    self.is_dragging_state.set(false);
                    false
                }
                _ => false,
            },
            _ => false,
        }
    }

    fn write_quads(&self, quad_buffer: &mut [crate::prelude::Graphic]) {
        /* Maybe push something here in Debug mode? */
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}