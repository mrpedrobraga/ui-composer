use std::task::Poll;
use {
    super::{Node, ReifiedNode},
    crate::app::node::UIEvent,
};

impl Node for () {
    type ReifiedType = ();

    fn reify(
        self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        gpu_resources: &super::Resources,
        renderers: &mut crate::winitwgpu::pipeline::Renderers,
    ) -> Self::ReifiedType {
    }
}

impl ReifiedNode for () {
    fn setup(&mut self, gpu_resources: &super::Resources) {}

    fn handle_window_event(
        &mut self,
        gpu_resources: &mut super::Resources,
        pipelines: &mut crate::winitwgpu::pipeline::Renderers,
        window_id: winit::window::WindowId,
        event: UIEvent,
    ) {
    }

    fn poll_processors(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}
