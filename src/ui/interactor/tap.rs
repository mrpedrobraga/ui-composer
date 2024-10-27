use super::Interactor;
use crate::ui::node::{LiveUINode, UIEvent, UINode};
use futures::SinkExt;
use futures_channel::mpsc::{self, Receiver, Sender};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use vek::{Rect, Vec2};

/// An Interactor that handles a user hovering over it with a cursor.
pub struct Tap {
    rect: Rect<f32, f32>,
    mouse_position: Vec2<f32>,
    tap_channel: Sender<Vec2<f32>>,
}

impl Tap {
    pub fn new(rect: Rect<f32, f32>) -> (Self, Receiver<Vec2<f32>>) {
        let (tap_channel, tap_receiver) = futures_channel::mpsc::channel(0);

        (
            Self {
                rect,
                mouse_position: Vec2::zero(),
                tap_channel,
            },
            tap_receiver,
        )
    }
}

impl Interactor for Tap {}
impl UINode for Tap {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None // Some(self.area))
    }
}
impl LiveUINode for Tap {
    fn handle_ui_event(&mut self, event: crate::ui::node::UIEvent) -> bool {
        match event {
            UIEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                self.mouse_position = Vec2::new(position.x, position.y).as_();
                false
            }
            UIEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => match (button, state) {
                (winit::event::MouseButton::Left, winit::event::ElementState::Pressed) => {
                    if (self.rect.contains_point(self.mouse_position)) {
                        pollster::block_on(self.tap_channel.send(self.mouse_position));
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            _ => false,
        }
    }

    fn push_quads(&self, quad_buffer: &mut [crate::prelude::Quad]) {
        /* Maybe push something here in Debug mode? */
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }

    fn poll_reactivity_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}
