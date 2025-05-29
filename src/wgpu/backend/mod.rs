use crate::prelude::Backend;
use crate::wgpu::pipeline::Renderers;
use pin_project::pin_project;
use std::marker::PhantomData;
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

impl<N, D> Backend for WGPUBackend<N, D> {
    type Tree = D;

    fn run(_node_tree: Self::Tree) {
        todo!()
    }

    fn poll_processors(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<()>> {
        todo!()
    }
}
