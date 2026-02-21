//! # Desktop (Winit)
//!
//! This module contains a [`Runner`] that runs applications
//! on many targets using `winit` under the hood.

use {
    crate::runner::WinitEnvironment,
    ui_composer_core::app::composition::{elements::Blueprint, UI},
};

pub mod gpu;
pub mod render;
pub mod runner;
pub mod window;
mod winit_uic_conversion;

pub trait WUI: UI<WinitEnvironment> {}
impl<T> WUI for T where T: UI<WinitEnvironment> {}

pub trait WinitBlueprint: Blueprint<WinitEnvironment, Element: Send> + Send {}
impl<T> WinitBlueprint for T where T: Blueprint<WinitEnvironment, Element: Send> + Send {}
