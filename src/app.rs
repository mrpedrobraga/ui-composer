use crate::{
    gpu::backend::{Backend as _, Node, WinitBackend as _, WinitWGPUBackend},
    prelude::*,
};
use std::sync::{Arc, Mutex};
use winit::application::ApplicationHandler;

/// App builder, receives a layout item with the entirety of your app.
pub struct App {}

impl App {
    // Creates and runs a new app.
    // For cross-platform compatibility, this should be called in the main thread,
    // and only once in your program.
    pub fn run<NodeDescriptorType: NodeDescriptor>(root_fragment: NodeDescriptorType)
    where
        NodeDescriptorType::ReifiedType: 'static,
    {
        WinitWGPUBackend::<NodeDescriptorType::ReifiedType>::run(root_fragment);
    }
}
