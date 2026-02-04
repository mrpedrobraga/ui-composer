//! # TUI
//!
//! This module contains a [`Runner`] that can run applications in a terminal.

pub mod runner;
pub mod nodes;
pub mod render;
pub mod signals;

pub use nodes::Terminal;
pub use render::Graphic;
use crate::app::composition::UI;
use crate::runners::tui::runner::TerminalEnvironment;

pub trait TUI: UI<TerminalEnvironment> {}
impl<T> TUI for T where T: UI<TerminalEnvironment> {}