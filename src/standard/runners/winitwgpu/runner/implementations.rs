use super::{EReify, Element};
use crate::standard::runners::wgpu::backend::WgpuResources;
use crate::standard::runners::wgpu::pipeline::WgpuRenderers;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
use crate::app::runner::AppContext;
use crate::state::process::Pollable;

impl EReify for () {
    type Output = ();

    fn reify(
        self,
        _event_loop: &ActiveEventLoop,
        _gpu_resources: &WgpuResources,
        _renderers: WgpuRenderers,
    ) -> Self::Output {
    }
}

impl Element for () {
    fn setup(&mut self, _gpu_resources: &WgpuResources) {}

    fn handle_window_event(
        &mut self,
        _gpu_resources: &mut WgpuResources,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
    }
}

impl<A, B> EReify for (A, B)
where
    A: EReify,
    B: EReify,
{
    type Output = (A::Output, B::Output);

    fn reify(
        self,
        _event_loop: &ActiveEventLoop,
        _gpu_resources: &WgpuResources,
        _renderers: WgpuRenderers,
    ) -> Self::Output {
        todo!("Figure out what to do with tuples of nodes...");
        // (
        //     self.0.reify(event_loop, gpu_resources, renderers),
        //     self.1.reify(event_loop, gpu_resources, renderers),
        // )
    }
}

impl<A, B> Element for (A, B)
where
    A: Element + Pollable<AppContext>,
    B: Element + Pollable<AppContext>,
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
