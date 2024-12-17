use futures_signals::signal_vec::MutableVec;
use vek::Rect;

use crate::{
    prelude::LayoutItem,
    ui::node::{UINode, UINodeDescriptor},
};

pub struct VecItem {
    rect: Rect<f32, f32>,
}

impl VecItem {
    pub fn new(rect: Rect<f32, f32>) -> Self {
        Self { rect }
    }
}

impl UINodeDescriptor for VecItem {
    const QUAD_COUNT: usize = 1;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        Some(self.rect)
    }
}

impl UINode for VecItem {
    fn handle_ui_event(&mut self, event: crate::ui::node::UIEvent) -> bool {
        // Handle UI events for each item!
        false
    }

    fn write_quads(&self, quad_buffer: &mut [crate::prelude::Quad]) {
        // TODO: Write no quads.
        quad_buffer[0] = crate::prelude::Quad::new(self.rect, vek::Rgb::green())
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }

    fn poll_processors(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>> {
        // TODO: Poll the processors for my items!
        std::task::Poll::Ready(Some(()))
    }
}
