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
pub use crate::app::input::items::*;
pub use crate::app::input::*;
pub use crate::app::runner::Runner;
use futures::executor::block_on;
use futures::join;

// MARK: Geometry and Layout
pub use crate::app::composition::layout::*;
pub use crate::geometry::*;

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
    pub fn run_custom<CustomRunner: Runner>(node_tree_descriptor: CustomRunner::AppBlueprint) {
        let mut runner = CustomRunner::run(node_tree_descriptor);

        use async_std::stream::StreamExt;
        println!("Requesting event stream...");
        let event_stream = runner.event_stream();
        println!("Event stream arrived.");
        let event_co = event_stream.for_each(|event| {
            println!("An event arrived = {:?}", event);
        });
        println!("Starting.");
        std::thread::scope(|s| {
            s.spawn(|| {
                block_on(async { join!(event_co) });
            });
            runner.main_loop();
        });

        println!("Done. Cleaning up...");
    }
}

#[cfg(all(feature = "winit", feature = "wgpu"))]
mod winit_wgpu {
    use crate::app::runner::Runner as _;
    use crate::standard::prelude::UIComposer;
    use crate::standard::runners::wgpu::backend::WgpuBackend;
    use crate::standard::runners::wgpu::pipeline::UIContext;
    use crate::standard::runners::winitwgpu::runner::{EReify, WinitWgpuRunner};
    use crate::state::process::Pollable;

    impl UIComposer {
        /// Creates and runs a new app in the default runner for the selected target.
        /// For cross-platform compatibility, this should be called in the main thread,
        /// and only once in your program.
        pub fn run<N: EReify + 'static>(node_tree_descriptor: N) {
            WinitWgpuRunner::<N>::run(node_tree_descriptor);
        }

        pub fn run2<N: EReify + 'static>(node_tree_descriptor: N)
        where
            N::Output: Pollable<UIContext>,
        {
            WgpuBackend::<N>::run(node_tree_descriptor);
        }
    }
}
