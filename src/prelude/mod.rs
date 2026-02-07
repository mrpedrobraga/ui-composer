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
    /// For cross-platform compatibility, this should be called in the main thread.
    pub fn run_custom<CustomRunner: Runner + Send>(app_blueprint: CustomRunner::AppBlueprint) {
        // TODO: `UIComposer` should pass an environment the app can use to create new processes.
        CustomRunner::run(app_blueprint);
    }
}

use crate::app::composition::elements::Blueprint;
use crate::runners::tui::runner::{TUIRunner, TerminalEnvironment};

impl UIComposer {
    pub fn run_tui(app_blueprint: impl Blueprint<TerminalEnvironment, Element: Send + 'static> + Send) {
        UIComposer::run_custom::<TUIRunner<_>>(app_blueprint);
    }
}