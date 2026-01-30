//! # Prelude (Work In Progress)
//!
//! Handy bundles to import to keep your import list tidy and get right to building.
//!
//! Usage:
//!
//! ```rust
//! use ui_composer::standard::prelude::*;
//! ```

// MARK: App
pub use crate::app::backend::Runner;
pub use crate::app::input::items::*;
pub use crate::app::input::*;

// MARK: Geometry and Layout
pub use crate::geometry::*;
pub use crate::geometry::layout::*;

// MARK: State
pub use crate::state::*;
pub use futures_signals::signal::{self, Signal};
pub use futures_signals::signal_vec::{self, SignalVec};

// Components
pub mod macros;
pub use crate::standard::*;

#[cfg(all(feature = "winit", feature = "wgpu"))]
pub use crate::standard::runners::winitwgpu::window::{Window, WindowAttributes};

/// This struct is the "entry point" of a UI Composer project.
pub struct UIComposer;

impl UIComposer {
    /// Creates and runs a new app in a given runner.
    /// For cross-platform compatibility, this should be called in the main thread,
    /// and only once in your program.
    pub fn run_custom<CustomBackend: Runner>(node_tree_descriptor: CustomBackend::App) {
        CustomBackend::run(node_tree_descriptor);
    }
}

#[cfg(all(feature = "winit", feature = "wgpu"))]
mod winit_wgpu {
    use crate::app::backend::Runner as _;
    use crate::standard::runners::wgpu::backend::WgpuBackend;
    use crate::standard::runners::wgpu::pipeline::UIContext;
    use crate::standard::runners::winitwgpu::runner::{Node, WinitWgpuRunner};
    use crate::standard::prelude::UIComposer;
    use crate::state::process::Pollable;

    impl UIComposer {
        /// Creates and runs a new app in the default runner for the selected target.
        /// For cross-platform compatibility, this should be called in the main thread,
        /// and only once in your program.
        pub fn run<N: Node + 'static>(node_tree_descriptor: N) {
            WinitWgpuRunner::<N>::run(node_tree_descriptor);
        }

        pub fn run2<N: Node + 'static>(node_tree_descriptor: N)
        where
            N::Output: Pollable<UIContext>,
        {
            WgpuBackend::<N>::run(node_tree_descriptor);
        }
    }
}