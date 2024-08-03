use super::node::{LiveUINode, UIEvent, UINode};

pub trait Interactor: UINode {}

pub struct Inspect();

impl Interactor for Inspect {}
impl UINode for Inspect {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        None
    }
}
impl LiveUINode for Inspect {
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        println!("Event: {:?}", event);
        false
    }

    fn push_quads(&self, quad_buffer: &mut [crate::prelude::Quad]) {
        /* No quads to push in release mode. But maybe in debug? */
    }

    fn poll_reactivity_change(
        &mut self,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>> {
        std::task::Poll::Ready(None)
    }
}
