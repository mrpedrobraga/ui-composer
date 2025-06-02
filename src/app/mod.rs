//! # Applications
//!
//! An application is a composition of [primitives::Node]s each which do something.
//!
//! To create your program, call `UIComposer::run` and give it a root [primitives::Node], like, for example, a [super::winitwgpu::window::WindowNode].
//!
//! ```ignore
//! use ui_composer::prelude::*;
//! UIComposer::run(Window(()));
//! ```
//!
//! This function _must_ be called in the main thread.
//!
//! ## Different Backends
//!
//! You can also call [UIComposer::run_custom] to give it a custom backend.

use backend::Backend;

pub mod backend;
pub mod implementations;
pub mod input;
pub mod primitives;

/// App builder, receives a layout item with the entirety of your app.
pub struct UIComposer;

impl UIComposer {
    /// Creates and runs a new app in a given backend.
    /// For cross-platform compatibility, this should be called in the main thread,
    /// and only once in your program.
    pub fn run_custom<CustomBackend: Backend>(node_tree_descriptor: CustomBackend::Tree) {
        CustomBackend::run(node_tree_descriptor);
    }
}

#[cfg(all(feature = "winit", feature = "wgpu"))]
mod winit_wgpu {
    use crate::app::primitives::Processor;
    use crate::wgpu::backend::WGPUBackend;
    use crate::wgpu::pipeline::UIReifyResources;
    use crate::winitwgpu::backend::WithWinit;
    use {
        super::{UIComposer, backend::Backend as _},
        crate::winitwgpu::backend::NodeDescriptor,
    };

    impl UIComposer {
        /// Creates and runs a new app in the default backend for the selected target.
        /// For cross-platform compatibility, this should be called in the main thread,
        /// and only once in your program.
        pub fn run<N: NodeDescriptor + 'static>(node_tree_descriptor: N) {
            WithWinit::<WGPUBackend<N>>::run(node_tree_descriptor);
        }

        pub fn run2<N: NodeDescriptor + 'static>(node_tree_descriptor: N)
        where
            N::Reified: Processor<UIReifyResources>,
        {
            WGPUBackend::<N>::run(node_tree_descriptor);
        }
    }
}
