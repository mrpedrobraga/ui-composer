//! This module uses [winit] and [wgpu] to render graphics with GPU acceleration.
//! Thus, this module supports all platforms that winit and wgpu supports.

pub mod backend;
pub mod dynamic;
pub mod image;
pub mod pipeline;
pub mod render_target;
pub mod texture;
pub mod view;
pub mod window;
