use crate::app::backend::NodeReifyResources;
use crate::app::primitives::Processor;
use crate::prelude::Backend;
use crate::winitwgpu::backend::NodeDescriptor;
use pin_project::pin_project;
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

/// A backend that can render our application to the GPU as well as forward interactive events to the app.
#[pin_project(project=WGPUBackendProj)]
pub struct WGPUBackend<A>
where
    A: NodeDescriptor,
{
    /// The node of the UI tree containing the entirety of the app, UI and behaviour.
    #[pin]
    pub tree: Arc<Mutex<A::Reified>>,
    pub gpu_resources: GPUResources,
}

/// The collection of resources the GPU backends use to
/// interact with the GPU.
pub struct GPUResources {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
}

impl<A> Backend for WGPUBackend<A>
where
    A: NodeDescriptor<Reified: Processor<NodeReifyResources>>,
{
    type Tree = A;

    fn run(_node_tree: Self::Tree) {
        unimplemented!()
    }

    fn poll_processors(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &mut NodeReifyResources,
    ) -> Poll<Option<()>> {
        let mut tree = self.tree.lock().unwrap();
        let tree = tree.deref_mut();
        let tree_pin = unsafe { Pin::new_unchecked(tree) };
        tree_pin.poll(cx, resources)
    }
}
