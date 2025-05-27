//! This module uses [winit] and [wgpu] to render graphics with GPU acceleration.
//! Thus, this module supports all platforms that winit and wgpu supports.

use crate::prelude::LayoutItem;

pub mod backend;
pub mod dynamic;
pub mod image;
pub mod pipeline;
pub mod portal;
pub mod render_target;
pub mod texture;
pub mod window;

pub mod components;

pub trait Item: LayoutItem {}
