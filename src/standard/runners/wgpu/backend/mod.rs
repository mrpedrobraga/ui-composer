use crate::app::backend::AppContext;
use crate::standard::runners::winitwgpu::runner::EReify;
use crate::app::backend::Runner;
use crate::state::process::Pollable;
use pin_project::pin_project;
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

/// A runner that can render our application to the GPU as well as forward interactive events to the app.
#[pin_project(project=WGPUBackendProj)]
pub struct WgpuBackend<A>
where
    A: EReify,
{
    /// The node of the UI tree containing the entirety of the app, UI and behaviour.
    #[pin]
    pub tree: Arc<Mutex<A::Output>>,
    pub gpu_resources: WgpuResources,
}

/// The collection of resources the GPU runners use to
/// interact with the GPU.
pub struct WgpuResources {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
}

impl<A> Runner for WgpuBackend<A>
where
    A: EReify<Output: Pollable<AppContext>>,
{
    type AppBlueprint = A;

    fn run(_node_tree: Self::AppBlueprint) {
        unimplemented!()
    }

    async fn event_loop(&self) {
        unimplemented!()
    }

    async fn react_loop(&self) {
        unimplemented!()
    }

    fn process(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &mut AppContext,
    ) -> Poll<Option<()>> {
        let mut tree = self.tree.lock().unwrap();
        let tree = tree.deref_mut();
        let tree_pin = unsafe { Pin::new_unchecked(tree) };
        tree_pin.poll(cx, resources)
    }
}
