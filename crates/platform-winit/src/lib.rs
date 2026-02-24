//! # Desktop (Winit)
//!
//! This module contains a [`Runner`] that runs applications
//! on many targets using `winit` under the hood.

use {
    crate::runner::WinitEnvironment,
    ui_composer_core::app::composition::{CompatibleWith, elements::Blueprint},
};

pub mod gpu;
pub mod render;
pub mod runner;
pub mod window;
mod winit_uic_conversion;

pub trait WinitUi: CompatibleWith<WinitEnvironment> {}
impl<T> WinitUi for T where T: CompatibleWith<WinitEnvironment> {}

pub trait WinitBlueprint:
    Blueprint<WinitEnvironment, Element: Send> + Send
{
}
impl<T> WinitBlueprint for T where
    T: Blueprint<WinitEnvironment, Element: Send> + Send
{
}

#[doc(hidden)]
pub mod prelude {
    pub use crate::WinitUi;
    pub use crate::runner::{WinitEnvironment, WinitRunner};
}
