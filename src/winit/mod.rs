//! Module for common concepts which interact with [`winit`], for the many winit backends.

use crate::wgpu::backend::WGPUBackend;
use {
    crate::{
        app::backend::{Backend, BackendProcessExecutor},
        winitwgpu::backend::NodeDescriptor,
    },
    futures_signals::signal::SignalFuture,
    std::sync::{Arc, Mutex},
    winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::WindowId},
};

/// A Backend that interacts with [`winit`]
pub trait WinitBackend: Backend + Send {
    type NodeTreeDescriptorType: NodeDescriptor + 'static;

    #[allow(async_fn_in_trait)]
    async fn create(
        event_loop: &ActiveEventLoop,
        tree_descriptor: Self::NodeTreeDescriptorType,
    ) -> (
        Arc<Mutex<Self>>,
        SignalFuture<
            BackendProcessExecutor<
                WGPUBackend<
                    <Self::NodeTreeDescriptorType as NodeDescriptor>::Reified,
                    Self::NodeTreeDescriptorType,
                >,
            >,
        >,
    );
    fn handle_resumed(&mut self);
    fn handle_window_event(&mut self, window_id: WindowId, event: WindowEvent);
}
