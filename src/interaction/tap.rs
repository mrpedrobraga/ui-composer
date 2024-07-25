use super::InteractorNode;
use futures_signals::signal::{Mutable, MutableSignal};
use vek::{Aabr, Rect, Vec2};
use winit::event::MouseButton;

#[derive(Clone)]
pub struct TapInteraction {
    tap: Mutable<Option<()>>,
    cursor_inside_bounds: bool,
    pub aabr: Aabr<f32>,
}

impl TapInteraction {
    pub fn new(rect: Rect<f32, f32>) -> Self {
        Self {
            tap: Mutable::new(None),
            cursor_inside_bounds: false,
            aabr: rect.into_aabr(),
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
                let position = Vec2::new(position.x as f32, position.y as f32);
                self.cursor_inside_bounds = self.aabr.contains_point(position);
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
