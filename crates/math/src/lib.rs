//! # Geometry
//!
//! Mathematical utilities for laying out things in space.

pub mod flow;
pub mod types;

/// Re-export of the `glam` crate for math types.
pub use glamour;
pub use palette;

pub mod prelude {
    pub use crate::flow::{
        CartesianFlow, CoordinateSystem, CurrentFlow, RelativeFlow, WritingFlow,
    };
    pub use crate::types::RectExt;

    pub use glamour::prelude::*;
    pub use palette::*;
}
