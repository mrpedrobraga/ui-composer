//! # TUI
//!
//! This module contains a [`Runner`] that can run applications in a terminal.

pub mod nodes;
pub mod render;
pub mod runner;

pub use nodes::Terminal;
pub use ui_composer_canvas as canvas;

use {
    crate::runner::TerminalEnvironment,
    ui_composer_core::app::composition::{CompatibleWith, elements::Blueprint},
};

pub trait Tui: CompatibleWith<TerminalEnvironment> {}
impl<T> Tui for T where T: CompatibleWith<TerminalEnvironment> {}

pub trait TuiBlueprint:
    Blueprint<TerminalEnvironment, Element: Send> + Send
{
}
impl<T> TuiBlueprint for T where
    T: Blueprint<TerminalEnvironment, Element: Send> + Send
{
}

pub mod prelude {
    pub use crate::Tui;
    pub use crate::nodes::Terminal;
    pub use crate::runner::{TUIRunner, TerminalEnvironment};
}
