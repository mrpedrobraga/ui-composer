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

/// This struct is the "entry point" of a UI Composer project.
pub struct UIComposer;

impl UIComposer {
    /// Creates and runs a new app in a given runner.
    /// For cross-platform compatibility, this should be called in the main thread,
    /// and only once in your program.
    pub fn run_custom<CustomRunner: Runner + Send>(app_blueprint: CustomRunner::AppBlueprint) {
        let mut runner = CustomRunner::run(app_blueprint);

        use async_std::stream::StreamExt;
        let event_stream = runner.event_stream();
        let main_loop = runner.main_loop();
        let mut on_update = runner.on_update();
        let event_co = event_stream.for_each(|event| {
            println!("An event arrived = {:?}", event);
            on_update();
        });
        std::thread::scope(|s| {
            s.spawn(|| {
                block_on(async { join!(event_co) });
            });
            main_loop();
        });

        println!("Done. Cleaning up...");
    }
}

use crate::app::composition::elements::Blueprint;
use crate::runners::tui::runner::{TUIRunner, TerminalEnvironment};

impl UIComposer {
    pub fn run_tui(app_blueprint: impl Blueprint<TerminalEnvironment, Element: Send + 'static> + Send) {
        UIComposer::run_custom::<TUIRunner<_>>(app_blueprint);
    }
}