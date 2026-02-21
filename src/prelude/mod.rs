//! # Prelude (Work In Progress)
//!
//! Handy bundles to import to keep your import list tidy and get right to building.
//!
//! Usage:
//!
//! ```rust
//! use ui_composer::standard::prelude::*;
//! ```

use ui_composer_core::app::{composition::elements::Blueprint, runner::Runner};
use ui_composer_platform_tui::runner::{TUIRunner, TerminalEnvironment};
use ui_composer_platform_winit::runner::{WinitEnvironment, WinitRunner};

pub mod macros;

/// This struct is the "entry point" of a UI Composer project.
pub struct UIComposer;

impl UIComposer {
    /// Creates and runs a new app in a given runner.
    /// For cross-platform compatibility, this should be called in the main thread.
    pub fn run_custom<CustomRunner: Runner + Send>(app_blueprint: CustomRunner::AppBlueprint) {
        CustomRunner::run(app_blueprint);
    }
}

impl UIComposer {
    pub fn run_tui(
        app_blueprint: impl Blueprint<TerminalEnvironment, Element: Send + 'static> + Send,
    ) {
        UIComposer::run_custom::<TUIRunner<_>>(app_blueprint);
    }

    pub fn run_winit(
        app_blueprint: impl Blueprint<WinitEnvironment, Element: Send + 'static> + Send,
    ) {
        UIComposer::run_custom::<WinitRunner<_>>(app_blueprint);
    }
}
