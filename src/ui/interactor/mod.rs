use super::node::{ItemDescriptor, UIEvent, UIItem};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
pub mod hover;
pub mod tap;
pub use hover::*;
pub use tap::*;

pub trait Interactor: ItemDescriptor {}
pub struct Inspect();

impl Interactor for Inspect {}
impl ItemDescriptor for Inspect {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        None
    }
}
impl UIItem for Inspect {
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        println!("Event: {:?}", event);
        false
    }

    fn write_quads(&self, quad_buffer: &mut [crate::prelude::Graphic]) {
        /* No quads to push in release mode. But maybe in debug? */
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}
