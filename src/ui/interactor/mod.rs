use super::node::{UIEvent, UINode, UINodeDescriptor};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub mod hover;
pub mod tap;
pub use hover::*;
pub use tap::*;

pub trait Interactor: UINodeDescriptor {}
pub struct Inspect();

impl Interactor for Inspect {}
impl UINodeDescriptor for Inspect {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        None
    }
}
impl UINode for Inspect {
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        println!("Event: {:?}", event);
        false
    }

    fn push_quads(&self, quad_buffer: &mut [crate::prelude::Quad]) {
        /* No quads to push in release mode. But maybe in debug? */
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}
