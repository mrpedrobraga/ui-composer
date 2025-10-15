//! # Applications
//!
//! An application is a composition of [building_blocks::BuildingBlock]s each which do something.
//!
//! To create your program, call `UIComposer::run` and give it a root [building_blocks::BuildingBlock], like, for example, a [super::winitwgpu::window::WindowNodeRe].
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
pub mod building_blocks;

/// This struct is the "entry point" of an UI Composer project.
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
    use crate::state::process::Pollable;
    use crate::wgpu::backend::WgpuBackend;
    use crate::wgpu::pipeline::UIContext;
    use crate::winitwgpu::backend::WinitWgpuBackend;
    use {
        super::{UIComposer, backend::Backend as _},
        crate::winitwgpu::backend::Node,
    };

    impl UIComposer {
        /// Creates and runs a new app in the default backend for the selected target.
        /// For cross-platform compatibility, this should be called in the main thread,
        /// and only once in your program.
        pub fn run<N: Node + 'static>(node_tree_descriptor: N) {
            WinitWgpuBackend::<N>::run(node_tree_descriptor);
        }

        pub fn run2<N: Node + 'static>(node_tree_descriptor: N)
        where
            N::Reified: Pollable<UIContext>,
        {
            WgpuBackend::<N>::run(node_tree_descriptor);
        }
    }
}
