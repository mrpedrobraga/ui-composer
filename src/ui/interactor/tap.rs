use super::Interactor;
use crate::state::Editable;
use crate::ui::node::{ItemDescriptor, UIEvent, UIItem};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use vek::{Rect, Vec2};

/// An Interactor that handles a user hovering over it with a cursor.
pub struct Tap {
    rect: Rect<f32, f32>,
    mouse_position_state: Editable<Option<Vec2<f32>>>,
    tap_state: Editable<Option<()>>,
}

impl Tap {
    pub fn new(
        rect: Rect<f32, f32>,
        mouse_position_state: Editable<Option<Vec2<f32>>>,
        tap_state: Editable<Option<()>>,
    ) -> Self {
        Self {
            rect,
            mouse_position_state,
            tap_state,
        }
    }
}

impl Interactor for Tap {}
impl ItemDescriptor for Tap {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None // Some(self.area))
    }
}
impl UIItem for Tap {
    fn handle_ui_event(&mut self, event: crate::ui::node::UIEvent) -> bool {
        match event {
            UIEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                self.mouse_position_state
                    .set(Some(Vec2::new(position.x, position.y).as_()));
                false
            }
            UIEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => match (button, state) {
                (winit::event::MouseButton::Left, winit::event::ElementState::Pressed) => {
                    if let Some(mouse_position) = self.mouse_position_state.get() {
                        if (self.rect.contains_point(mouse_position)) {
                            self.tap_state.set(Some(()));
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
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
