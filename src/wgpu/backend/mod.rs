use crate::wgpu::pipeline::Renderers;
use pin_project::pin_project;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

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
