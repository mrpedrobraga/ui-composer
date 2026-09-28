//! # TUI
//!
//! This module contains a [`Runner`] that can run applications in a terminal.

pub mod items;
pub mod render;
pub mod runner;

pub use items::Terminal;
pub use ui_composer_canvas as canvas;

use {
    crate::runner::TerminalEnvironment,
    ui_composer_core::app::composition::{elements::Blueprint, CompatibleWith},
};

pub trait Tui: CompatibleWith<TerminalEnvironment> {}
impl<T> Tui for T where T: CompatibleWith<TerminalEnvironment> {}

pub trait TuiBlueprint: Blueprint<TerminalEnvironment, Output: Send> + Send {}
impl<T> TuiBlueprint for T where T: Blueprint<TerminalEnvironment, Output: Send> + Send {}

pub mod prelude {
    pub use crate::items::Terminal;
    pub use crate::runner::{TerminalEnvironment, TuiRunner};
    pub use crate::Tui;
}
