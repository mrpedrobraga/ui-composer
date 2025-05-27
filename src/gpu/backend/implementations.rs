use super::{Node, ReifiedNode};
use std::task::Poll;

// -- The Empty Node --

impl Node for () {
    type ReifiedType = ();

    fn reify(
        self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        gpu_resources: &super::GPUResources,
        renderers: &mut crate::gpu::pipeline::Renderers,
    ) -> Self::ReifiedType {
        ()
    }
}

impl ReifiedNode for () {
    fn setup(&mut self, gpu_resources: &super::GPUResources) {
        ()
    }

    fn handle_window_event(
        &mut self,
        gpu_resources: &mut super::GPUResources,
        pipelines: &mut crate::gpu::pipeline::Renderers,
        window_id: winit::window::WindowId,
        event: crate::prelude::UIEvent,
    ) {
        ()
    }

    fn poll_processors(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}
