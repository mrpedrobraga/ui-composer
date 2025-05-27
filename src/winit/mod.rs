//! Module for common concepts which interact with [`winit`], for the many winit backends.

use {
    crate::{
        app::{backend::Backend, node::UIEvent},
        winitwgpu::backend::{BackendProcessExecutor, Node, WinitWGPUBackend},
    },
    futures_signals::signal::SignalFuture,
    std::sync::{Arc, Mutex},
    winit::{event_loop::ActiveEventLoop, window::WindowId},
};

/// A Backend that interacts with [`winit`]
pub trait WinitBackend: Backend + Send {
    type NodeTreeDescriptorType: Node + 'static;

    #[allow(async_fn_in_trait)]
    async fn create(
        event_loop: &ActiveEventLoop,
        tree_descriptor: Self::NodeTreeDescriptorType,
    ) -> (
        Arc<Mutex<Self>>,
        SignalFuture<BackendProcessExecutor<WinitWGPUBackend<Self::NodeTreeDescriptorType>>>,
    );
    fn handle_resumed(&mut self);
    fn handle_window_event(&mut self, window_id: WindowId, event: UIEvent);
}
