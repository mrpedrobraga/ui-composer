use crate::{
    gpu::backend::{Backend, Node, WinitBackend as _, WinitWGPUBackend},
    prelude::*,
};
use std::sync::{Arc, Mutex};
use winit::application::ApplicationHandler;

type DefaultBackend<Nd> = WinitWGPUBackend<Nd>;

/// App builder, receives a layout item with the entirety of your app.
pub struct App {}

impl App {
    // Creates and runs a new app in the default backend for the selected target.
    // For cross-platform compatibility, this should be called in the main thread,
    // and only once in your program.
    pub fn run<Nd: NodeDescriptor + 'static>(node_tree_descriptor: Nd) {
        DefaultBackend::run(node_tree_descriptor);
    }

    // Creates and runs a new app in a given backend.
    // For cross-platform compatibility, this should be called in the main thread,
    // and only once in your program.
    pub fn run_custom<CustomBackend: Backend>(node_tree_descriptor: CustomBackend::NodeTreeType) {
        CustomBackend::run(node_tree_descriptor);
    }
}
