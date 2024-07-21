use super::InteractorNode;
use crate::geometry::aabb::AABB;
use futures_signals::signal::{Mutable, MutableSignal};
use winit::event::MouseButton;

pub struct TapInteraction {
    tap: Mutable<Option<()>>,
    cursor_inside_bounds: bool,
    pub aabb: AABB,
}

impl TapInteraction {
    pub fn new(aabb: AABB) -> Self {
        Self {
            tap: Mutable::new(None),
            cursor_inside_bounds: false,
            aabb,
        }
    }

    /// Returns a signal to the internal state of this node.
    pub fn get_signal(&self) -> MutableSignal<Option<()>> {
        self.tap.signal()
    }
}

impl InteractorNode for TapInteraction {
    fn handle_event(&mut self, event: winit::event::WindowEvent) -> bool {
        match event {
            winit::event::WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                let position = (position.x as i32, position.y as i32);
                self.cursor_inside_bounds = self.aabb.contains_point(position);
                false
            }
            winit::event::WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                if let MouseButton::Left = button {
                    if state.is_pressed() && self.cursor_inside_bounds {
                        self.tap.set(Some(()));
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
