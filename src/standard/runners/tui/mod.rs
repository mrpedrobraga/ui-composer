//! # TUI
//!
//! This module contains a [`TUIRunner`] that can run applications in a terminal.

pub mod runner;
pub mod nodes;
pub mod render;
pub mod signals;

pub use nodes::Terminal;
pub use render::Graphic;

use crate::app::composition::algebra::Bubble;
use crate::app::input::Event;
use crate::runners::tui::render::Canvas;
use crate::state::process::Pollable;
use vek::{Rect, Rgba};

pub trait Element: Send + Bubble<Event, bool> + Pollable<()> {
    fn setup(&mut self);

    fn draw<C>(&self, canvas: &mut C, rect: Rect<u16, u16>)
    where
        C: Canvas<Pixel = Rgba<u8>>;
}
