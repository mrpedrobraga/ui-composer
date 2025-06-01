#![allow(unused)]

use crate::app::primitives::Processor;
use vek::Vec2;
use {
    super::super::{Event, InputItem},
    crate::{
        app::{
            input::{ButtonState, MouseButton},
            primitives::Primitive,
        },
        prelude::CursorEvent,
    },
    futures_signals::signal::Mutable,
    vek::Rect,
};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum DragState {
    #[default]
    None,
    Hovering,
    Dragging,
}

/// An Interactor that handles a user dragging the window.
#[derive(Debug, Clone)]
pub struct Drag {
    rect: Rect<f32, f32>,
    drag_state: Mutable<DragState>,
    mouse_position: Mutable<Vec2<f32>>,

    // The thing we actually care about...
    displacement: Mutable<Vec2<f32>>,
}

impl Drag {
    pub fn new(
        rect: Rect<f32, f32>,
        drag_state: Mutable<DragState>,
        mouse_position: Mutable<Vec2<f32>>,
        displacement: Mutable<Vec2<f32>>,
    ) -> Self {
        Self {
            rect,
            drag_state,
            mouse_position,

            displacement,
        }
    }
}

impl InputItem for Drag {}

impl<Res> Primitive<Res> for Drag {
    fn handle_event(&mut self, event: Event) -> bool {
        if let Event::Cursor { id, event } = event {
            match (event, self.drag_state.get()) {
                (CursorEvent::Moved { position }, DragState::None) => {
                    self.mouse_position.set(position);
                    if self.rect.contains_point(position) {
                        self.drag_state.set(DragState::Hovering);
                    }
                    true
                }
                (CursorEvent::Moved { position }, DragState::Hovering) => {
                    self.mouse_position.set(position);
                    if !self.rect.contains_point(position) {
                        self.drag_state.set(DragState::None);
                    }
                    false
                }
                (CursorEvent::Moved { position }, DragState::Dragging) => {
                    if self.rect.contains_point(position) {
                        let delta = position - self.mouse_position.get();
                        *self.displacement.lock_mut() += delta;
                    } else {
                        // If the mouse leaves the drag area, drag stops.
                        // maybe this shouldn't be here.
                        self.drag_state.set(DragState::None);
                    }
                    self.mouse_position.set(position);
                    true
                }
                (CursorEvent::Exited, DragState::Dragging | DragState::Hovering) => {
                    self.drag_state.set(DragState::None);
                    false
                }
                (CursorEvent::Button(button, state), DragState::Hovering) => {
                    if let (MouseButton::Left, ButtonState::Pressed) = (button, state) {
                        self.drag_state.set(DragState::Dragging);
                        true
                    } else {
                        false
                    }
                }
                (CursorEvent::Button(button, state), DragState::Dragging) => {
                    if let (MouseButton::Left, ButtonState::Released) = (button, state) {
                        self.drag_state.set(DragState::Hovering);
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            }
        } else {
            false
        }
    }
}

impl<Res> Processor<Res> for Drag {}
