use crate::app::backend::AppContext;
use crate::standard::backends::winitwgpu::backend::Node;
use crate::app::backend::Backend;
use crate::state::process::Pollable;
use pin_project::pin_project;
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

/// A backend that can render our application to the GPU as well as forward interactive events to the app.
#[pin_project(project=WGPUBackendProj)]
pub struct WgpuBackend<A>
where
    A: Node,
{
    /// The node of the UI tree containing the entirety of the app, UI and behaviour.
    #[pin]
    pub tree: Arc<Mutex<A::Reified>>,
    pub gpu_resources: WgpuResources,
}

/// The collection of resources the GPU backends use to
/// interact with the GPU.
pub struct WgpuResources {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
}

impl<A> Backend for WgpuBackend<A>
where
    A: Node<Reified: Pollable<AppContext>>,
{
    type Tree = A;

    fn run(_node_tree: Self::Tree) {
        unimplemented!()
    }

    fn poll_processors(
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
