//! # Applications
//!
//! This module has two concepts: [`BuildingBlock`]s and [`Backend`], as well as some utilities.
//!
//! When you are writing an application with UI Composer, you will work with [`Reifiable`]s:
//! those are types that *describe* parts of your application, but don't actually *do* anything.
//!
//! The [`Window`] item shown here produces something that describes a winit window but
//! doesn't actually have any references to any operating system resources.
//!
//! ```rust
//! # use ui_composer::prelude::*;
//! Window(())
//! ```
//!
//! When you pass your app description to a [`Backend`], it will be [`Reifiable::reify`]ed into
//! types that do have access to system resources (for example, `Window` here will get a handle
//! to a winit window).
//!
//! ## Backend-generic apps
//!
//! The [`Reifiable`] trait has a generic parameter `Context` — this means that you can implement
//! the trait several times for the same type... which means the same application can be reified
//! by two or more distinct backends.
//!
//! For example, you could have the same application run on the GPU, or in the CPU with multiple threads,
//! or in a CPU single-threaded, or in the terminal, or in the web, etc, though not all [`Reifiable`]s
//! work with all backends.

pub mod backend;
pub mod building_blocks;
pub mod input;

use backend::Backend;
use building_blocks::{BuildingBlock, Reifiable};

