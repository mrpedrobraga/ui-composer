//! # Geometry
//!
//! Mathematical utilities for laying out things in space.

pub mod flow;
pub mod geometry_ext;
pub use geometry_ext::RectExt;

pub mod prelude {
    pub use crate::flow::{
        CartesianFlow, CoordinateSystem, CurrentFlow, RelativeFlow, WritingFlow,
    };
    pub use crate::geometry_ext::RectExt as _;
    pub use vek;
    pub use vek::*;
}
