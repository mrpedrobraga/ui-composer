//! A library for creating fast cross-platform user interfaces.

#[doc(hidden)]
#[rust_analyzer::completions(ignore_flyimport)]
pub mod prelude;

pub mod app;
pub mod geometry;
pub mod layout;
pub mod state;

#[cfg(feature = "winit")]
pub mod winit;

pub mod components;

#[cfg(feature = "wgpu")]
pub mod wgpu;

#[cfg(all(feature = "winit", feature = "wgpu"))]
pub mod winitwgpu;

#[cfg(feature = "tui")]
pub mod tui;
