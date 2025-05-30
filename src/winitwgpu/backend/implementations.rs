use super::{Node, NodeDescriptor};
use crate::wgpu::backend::Resources;
use crate::wgpu::pipeline::Renderers;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

impl NodeDescriptor for () {
    type Reified = ();

    fn reify(
        self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _gpu_resources: &super::Resources,
        _renderers: &mut crate::wgpu::pipeline::Renderers,
    ) -> Self::Reified {
    }
}

impl Node for () {
    fn setup(&mut self, _gpu_resources: &super::Resources) {}

    fn handle_window_event(
        &mut self,
        _gpu_resources: &mut super::Resources,
        _pipelines: &mut crate::wgpu::pipeline::Renderers,
        _window_id: winit::window::WindowId,
        _event: WindowEvent,
    ) {
    }
}

impl<A, B> NodeDescriptor for (A, B)
where
    A: NodeDescriptor,
    B: NodeDescriptor,
{
    type Reified = (A::Reified, B::Reified);

    fn reify(
        self,
        event_loop: &ActiveEventLoop,
        gpu_resources: &Resources,
        renderers: &mut Renderers,
    ) -> Self::Reified {
        (
            self.0.reify(event_loop, gpu_resources, renderers),
            self.1.reify(event_loop, gpu_resources, renderers),
        )
    }
}

impl<A, B> Node for (A, B)
where
    A: Node,
    B: Node,
{
    fn setup(&mut self, gpu_resources: &Resources) {
        self.0.setup(gpu_resources);
        self.1.setup(gpu_resources);
    }

    fn handle_window_event(
        &mut self,
        gpu_resources: &mut Resources,
        pipelines: &mut Renderers,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        self.0
            .handle_window_event(gpu_resources, pipelines, window_id, event.clone());
        self.1
            .handle_window_event(gpu_resources, pipelines, window_id, event);
    }
}
