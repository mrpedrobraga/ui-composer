use super::InteractorNode;
use futures_signals::signal::{Mutable, MutableSignal};
use vek::{Aabr, Vec2};

#[derive(Clone)]
pub struct HoverInteraction {
    is_hovered: Mutable<bool>,
    pub aabb: Aabr<i32>,
}

impl HoverInteraction {
    pub fn new(aabb: Aabr<i32>) -> Self {
        Self {
            is_hovered: Mutable::new(false),
            aabb,
        }
    }

    /// Returns a signal to the internal state of this node.
    pub fn get_signal(&self) -> MutableSignal<bool> {
        self.is_hovered.signal()
    }
}

impl InteractorNode for HoverInteraction {
    fn handle_event(&mut self, event: winit::event::WindowEvent) -> bool {
        match event {
            winit::event::WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                let position = Vec2::new(position.x as i32, position.y as i32);
                let is_hovered = self.aabb.contains_point(position);
                if self.is_hovered.get() != is_hovered {
                    self.is_hovered.set(is_hovered);
                }
                true
            }
            winit::event::WindowEvent::CursorLeft { device_id } => {
                if self.is_hovered.get() {
                    self.is_hovered.set(false);
                }
                false
            }
            _ => false,
        }
    }
}
