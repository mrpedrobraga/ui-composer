//! # Desktop (Winit)
//!
//! This module contains a [`Runner`] that runs applications
//! on many targets using `winit` under the hood.

use {
    crate::runner::DesktopEnvironment,
    ui_composer_core::app::composition::{elements::Blueprint, CompatibleWith},
};

pub mod gpu;
pub mod render;
pub mod runner;
pub mod window;
mod winit_uic_conversion;

pub use wgpu;
pub use winit;

pub trait DesktopUi: CompatibleWith<DesktopEnvironment> {}
impl<T> DesktopUi for T where T: CompatibleWith<DesktopEnvironment> {}

pub trait DesktopBlueprint: Blueprint<DesktopEnvironment, Output: Send> + Send {}
impl<T> DesktopBlueprint for T where T: Blueprint<DesktopEnvironment, Output: Send> + Send {}

#[doc(hidden)]
pub mod prelude {
    pub use crate::runner::{DesktopEnvironment, DesktopPlatform};
    pub use crate::DesktopUi;
    pub use crate::window::Window;
}
