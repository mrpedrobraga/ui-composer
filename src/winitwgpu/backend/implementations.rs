use {
    super::{Node, ReifiedNode},
    crate::app::node::UIEvent,
};
use {std::task::Poll, winit::event::WindowEvent};

impl Node for () {
    type Reified = ();

    fn reify(
        self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _gpu_resources: &super::Resources,
        _renderers: &mut crate::winitwgpu::pipeline::Renderers,
    ) -> Self::Reified {
    }
}

impl ReifiedNode for () {
    fn setup(&mut self, _gpu_resources: &super::Resources) {}

    fn handle_window_event(
        &mut self,
        _gpu_resources: &mut super::Resources,
        _pipelines: &mut crate::winitwgpu::pipeline::Renderers,
        _window_id: winit::window::WindowId,
        _event: WindowEvent,
    ) {
    }

    fn poll_processors(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}
