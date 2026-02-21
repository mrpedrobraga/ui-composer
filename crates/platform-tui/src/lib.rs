//! # TUI
//!
//! This module contains a [`Runner`] that can run applications in a terminal.

pub mod blueprints;
pub mod nodes;
pub mod render;
pub mod runner;

pub use nodes::Terminal;
pub use render::Graphic;
pub use ui_composer_canvas as canvas;

use {
    crate::runner::TerminalEnvironment,
    ui_composer_core::app::composition::{elements::Blueprint, UI},
};

pub trait TUI: UI<TerminalEnvironment> {}
impl<T> TUI for T where T: UI<TerminalEnvironment> {}

pub trait TUIBlueprint: Blueprint<TerminalEnvironment, Element: Send> + Send {}
impl<T> TUIBlueprint for T where T: Blueprint<TerminalEnvironment, Element: Send> + Send {}
