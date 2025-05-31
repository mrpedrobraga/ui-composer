use super::{Node, NodeDescriptor};
use crate::wgpu::backend::GPUResources;
use crate::wgpu::pipeline::Renderers;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

impl NodeDescriptor for () {
    type Reified = ();

    fn reify(
        self,
        _event_loop: &ActiveEventLoop,
        _gpu_resources: &GPUResources,
        _renderers: Renderers,
    ) -> Self::Reified {
    }
}

impl Node for () {
    fn setup(&mut self, _gpu_resources: &GPUResources) {}

    fn handle_window_event(
        &mut self,
        _gpu_resources: &mut GPUResources,
        _window_id: WindowId,
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
        _event_loop: &ActiveEventLoop,
        _gpu_resources: &GPUResources,
        _renderers: Renderers,
    ) -> Self::Reified {
        todo!("Figure out what to do with tuples of nodes...");
        // (
        //     self.0.reify(event_loop, gpu_resources, renderers),
        //     self.1.reify(event_loop, gpu_resources, renderers),
        // )
    }
}

impl<A, B> Node for (A, B)
where
    A: Node,
    B: Node,
{
    fn setup(&mut self, gpu_resources: &GPUResources) {
        self.0.setup(gpu_resources);
        self.1.setup(gpu_resources);
    }

    fn handle_window_event(
        &mut self,
        gpu_resources: &mut GPUResources,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let e = event.clone();
        self.0.handle_window_event(gpu_resources, window_id, e);
        self.1.handle_window_event(gpu_resources, window_id, event);
    }
}
