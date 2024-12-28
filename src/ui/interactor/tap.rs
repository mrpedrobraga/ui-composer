use super::Interactor;
use crate::prelude::Action;
use crate::state::Mutable;
use crate::ui::node::{ItemDescriptor, UIEvent, UIItem};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use vek::{Rect, Vec2};

/// An Interactor that handles a user hovering over it with a cursor.
pub struct Tap<A: Action> {
    rect: Rect<f32, f32>,
    mouse_position_state: Mutable<Option<Vec2<f32>>>,
    tap_action: A,
}

impl<A> Tap<A>
where
    A: Action,
{
    pub fn new(
        rect: Rect<f32, f32>,
        mouse_position_state: Mutable<Option<Vec2<f32>>>,
        tap_state: A,
    ) -> Self {
        Self {
            rect,
            mouse_position_state,
            tap_action: tap_state,
        }
    }
}

impl<A> Interactor for Tap<A> where A: Action + Send {}
impl<A> ItemDescriptor for Tap<A>
where
    A: Action + Send,
{
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None // Some(self.area))
    }
}
impl<A> UIItem for Tap<A>
where
    A: Action + Send,
{
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
                            self.tap_action.trigger();
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
