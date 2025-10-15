use super::{NodeRe, Node};
use crate::wgpu::backend::WgpuResources;
use crate::wgpu::pipeline::WgpuRenderers;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

impl Node for () {
    type Reified = ();

    fn reify(
        self,
        _event_loop: &ActiveEventLoop,
        _gpu_resources: &WgpuResources,
        _renderers: WgpuRenderers,
    ) -> Self::Reified {
    }
}

impl NodeRe for () {
    fn setup(&mut self, _gpu_resources: &WgpuResources) {}

    fn handle_window_event(
        &mut self,
        _gpu_resources: &mut WgpuResources,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
    }
}

impl<A, B> Node for (A, B)
where
    A: Node,
    B: Node,
{
    type Reified = (A::Reified, B::Reified);

    fn reify(
        self,
        _event_loop: &ActiveEventLoop,
        _gpu_resources: &WgpuResources,
        _renderers: WgpuRenderers,
    ) -> Self::Reified {
        todo!("Figure out what to do with tuples of nodes...");
        // (
        //     self.0.reify(event_loop, gpu_resources, renderers),
        //     self.1.reify(event_loop, gpu_resources, renderers),
        // )
    }
}

impl<A, B> NodeRe for (A, B)
where
    A: NodeRe,
    B: NodeRe,
{
    fn setup(&mut self, gpu_resources: &WgpuResources) {
        self.0.setup(gpu_resources);
        self.1.setup(gpu_resources);
    }

    fn handle_window_event(
        &mut self,
        gpu_resources: &mut WgpuResources,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let e = event.clone();
        self.0.handle_window_event(gpu_resources, window_id, e);
        self.1.handle_window_event(gpu_resources, window_id, event);
    }
}
