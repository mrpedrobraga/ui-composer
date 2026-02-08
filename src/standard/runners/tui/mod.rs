//! # TUI
//!
//! This module contains a [`Runner`] that can run applications in a terminal.

pub mod nodes;
pub mod render;
pub mod runner;

use crate::app::composition::UI;
use crate::app::composition::elements::Blueprint;
use crate::runners::tui::runner::TerminalEnvironment;
pub use nodes::Terminal;
pub use render::Graphic;

pub trait TUI: UI<TerminalEnvironment> {}
impl<T> TUI for T where T: UI<TerminalEnvironment> {}

pub trait TUIBlueprint: Blueprint<TerminalEnvironment, Element: Send> + Send {}
impl<T> TUIBlueprint for T where T: Blueprint<TerminalEnvironment, Element: Send> + Send {}
