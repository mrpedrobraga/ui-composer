use super::Interactor;
use crate::ui::node::{ItemDescriptor, UIEvent, UIItem};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use vek::{Rect, Vec2};

/// An Interactor that handles a user hovering over it with a cursor.
pub struct Hover {
    rect: Rect<f32, f32>,
    is_hovered_state: Mutable<bool>,
}

impl Hover {
    pub fn new(rect: Rect<f32, f32>) -> Self {
        Self {
            rect,
            is_hovered_state: Mutable::new(false),
        }
    }

    /// Gets a signal to the hover interactor, so that you can react to it being hovered.
    pub fn signal(&self) -> impl Signal<Item = bool> {
        self.is_hovered_state.signal().dedupe()
    }
}

impl Interactor for Hover {}
impl ItemDescriptor for Hover {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None // Some(self.area))
    }
}
impl UIItem for Hover {
    fn handle_ui_event(&mut self, event: crate::ui::node::UIEvent) -> bool {
        match event {
            UIEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                let position = Vec2::new(position.x as f32, position.y as f32);
                self.is_hovered_state
                    .set(self.rect.contains_point(position));
                true
            }
            UIEvent::CursorLeft { device_id: _ } => {
                self.is_hovered_state.set(false);
                false
            }
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
