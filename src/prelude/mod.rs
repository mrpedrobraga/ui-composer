//! # Prelude (Work In Progress)
//!
//! Handy bundles to import to keep your import list tidy and get right to building.
//!
//! Usage:
//!
//! ```rust
//! use ui_composer::standard::prelude::*;
//! ```

pub mod macros;

/* Core */
pub use ui_composer_core::prelude::*;

/* Terminal target */
pub use ui_composer_platform_tui::prelude::*;

/* Winit target */
pub use ui_composer_platform_winit::prelude::*;

/* Geometry */
pub use ui_composer_math::prelude::*;

/* State */
pub use ui_composer_state::prelude::*;

/* Re-exported crates for convenience. */
pub use ui_composer_state::futures_signals;

/* Macros */
pub use macros::list;
pub use ui_composer_view_macro::view;
pub use uix::uix;
