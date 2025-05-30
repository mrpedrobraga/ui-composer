use crate::app::primitives::Processor;
use crate::prelude::Backend;
use crate::wgpu::pipeline::Renderers;
use pin_project::pin_project;
use std::marker::PhantomData;
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

/// A backend that can render our application to the GPU as well as forward interactive events to the app.
#[pin_project(project=WGPUBackendProj)]
pub struct WGPUBackend<N, D> {
    /// The node of the UI tree containing the entirety of the app, UI and behaviour.
    #[pin]
    pub tree: Arc<Mutex<N>>,
    pub gpu_resources: Resources,
    pub renderers: Renderers,

    pub _descriptor: PhantomData<D>,
}

/// The collection of resources the GPU backends use to
/// interact with the GPU.
pub struct Resources {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
}

impl<A, D> Backend for WGPUBackend<A, D>
where
    A: Processor,
{
    type Tree = D;

    fn run(_node_tree: Self::Tree) {
        unimplemented!()
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        let mut tree = self.tree.lock().unwrap();
        let tree = tree.deref_mut();
        let tree_pin = unsafe { Pin::new_unchecked(tree) };
        tree_pin.poll(cx)
    }
}
