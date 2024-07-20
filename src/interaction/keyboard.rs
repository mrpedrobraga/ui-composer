use super::InteractorNode;

pub struct Keyboard {}

impl InteractorNode for Keyboard {
    fn handle_event(&mut self, event: winit::event::WindowEvent) -> bool {
        todo!()
    }
}
