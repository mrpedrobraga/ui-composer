use super::node::{LiveUINode, UIEvent, UINode};

pub trait Interactor: UINode {}

pub struct Inspect();

impl Interactor for Inspect {}
impl UINode for Inspect {
    const PRIMITIVE_COUNT: usize = 0;
}
impl LiveUINode for Inspect {
    fn handle_event(&mut self, event: UIEvent) -> bool {
        println!("Event: {:?}", event);
        false
    }
}
