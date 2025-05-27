//! # Applications
//!
//! An application is a composition of [Nodes] each which do something.
//!
//! To create your program, call `UIComposer::run` and give it a root [Node], like, for example, a [Window].
//!
//! ```rust
//! use ui_composer::prelude::*;
//! UIComposer::run(Window(()));
//! ```
//!
//! This function _must_ be called in the main thread.
//!
//! ## Different Backends
//!
//! You can also call [UIComposer::run_custom] to give it a custom backend.
//! By default, apps use [WinitWGPUBackend], running on the GPU.

use crate::winitwgpu::backend::{Node, WinitWGPUBackend};
use backend::Backend;

pub mod backend;
pub mod node;

/// The default backend this crate runs.
/// I might change it depending on the target.
type DefaultBackend<N> = WinitWGPUBackend<N>;

/// App builder, receives a layout item with the entirety of your app.
pub struct UIComposer;

impl UIComposer {
    /// Creates and runs a new app in the default backend for the selected target.
    /// For cross-platform compatibility, this should be called in the main thread,
    /// and only once in your program.
    pub fn run<N: Node + 'static>(node_tree_descriptor: N) {
        DefaultBackend::run(node_tree_descriptor);
    }

    /// Creates and runs a new app in a given backend.
    /// For cross-platform compatibility, this should be called in the main thread,
    /// and only once in your program.
    pub fn run_custom<CustomBackend: Backend>(node_tree_descriptor: CustomBackend::NodeTreeType) {
        CustomBackend::run(node_tree_descriptor);
    }
}
