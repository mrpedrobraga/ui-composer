//! # Flow
//!
//! Containers lay out their elements in a specific kind of way.
//! The direction of flow of a container depends on many factors: app design,
//! but also _culture_ and language. For example, if you are arranging items "in order",
//! some cultures interpret that as "left to right" but others interpret it as "right to left".
//!
//! For accessibility, [FlowDirection] is _semantic_ by default,
//! using ideas of "previous" and "next" instead of "left" and "right".
//!
//! There are three kinds of flow direction:
//!
//! ### [FlowDirection::Writing]
//! This is the one you probably want. If you want to lay items "first" to "last", use this.
//!
//! Containers will lay the items in the order that characters flow in the current locale.
//! In `en_US`, horizontal layout lay left to right, and vertical standard lay top to bottom.
//!
//! There's also the idea of "Axis" and "Cross-Axis." The axis is the direction that the characters flow,
//! the cross-axis is the direction in which the text grows when it wraps.
//!
//! ### [FlowDirection::Relative]
//! This is for composing nested layout. It specifies a flow relative to the parent flow.
//!
//! For example, if the parent from is [WritingFlowDirection::WritingAxisForward] and you use
//! [RelativeFlowDirection::MainAxisBackward], this is the equivalent of using
//! [WritingFlowDirection::WritingAxisBackward].
//!
//! ### [FlowDirection::Cartesian]
//! This should be your last resort — it's absolute flow. Use it only for things that absolutely
//! require things to be shown the same way for everybody. Compasses, drawings, etc.

use crate::geometry::layout::hints::ParentHints;
use cgmath::{BaseFloat, Matrix4};
use vek::{Mat3, Vec2};

pub mod allocators;

/// The direction that a container spreads its items in.
#[derive(Debug, Clone, Copy)]
pub enum FlowDirection {
    /// Flow aligned with the writing flow (usually based on locale).
    Writing(WritingFlowDirection),
    /// Flow aligned relative to the parent flow.
    Relative(RelativeFlowDirection),
    /// Geometrically, absolutely defined flow.
    Cartesian(CartesianFlowDirection),
}

/// Specifies the flow direction in a geometric/absolute fashion.
/// Use it for things like graphics or symbols.
#[derive(Debug, Clone, Copy)]
pub enum CartesianFlowDirection {
    ///→
    LeftToRight,
    ///←
    RightToLeft,
    ///↓
    TopToBottom,
    ///↑
    BottomToTop,
}

impl CartesianFlowDirection {
    // Inverts this flow.
    fn inverse(&self) -> Self {
        match self {
            Self::LeftToRight => Self::RightToLeft,
            Self::RightToLeft => Self::LeftToRight,
            Self::TopToBottom => Self::BottomToTop,
            Self::BottomToTop => Self::TopToBottom,
        }
    }
}

/// Specifies a flow direction relative to the current "flow context."
#[derive(Debug, Clone, Copy)]
pub enum RelativeFlowDirection {
    /// Aligned with parent flow. Default: →.
    MainAxisForward,
    /// Opposite of parent flow. Default: ←.
    MainAxisBackwards,
    /// Aligned with parent cross flow. Default: ↓.
    CrossAxisForward,
    /// Opposite of parent cross flow. Default: ↑.
    CrossAxisBackwards,
}

/// Specifies a flow direction taking the locale into consideration,
/// whenever the idea of the elements of a container being "ordered"
/// comes into play.
#[derive(Debug, Clone, Copy)]
pub enum WritingFlowDirection {
    /// Aligned with writing flow. en-us: →.
    WritingAxisForward,
    /// Opposite of writing flow. en-us: ←.
    WritingAxisBackwards,
    /// Aligned with writing cross flow. en-us: ↓.
    WritingCrossAxisForward,
    /// Opposite of writing cross flow. en-us: ↑.
    WritingCrossAxisBackwards,
}

impl From<CartesianFlowDirection> for FlowDirection {
    fn from(val: CartesianFlowDirection) -> Self {
        FlowDirection::Cartesian(val)
    }
}

impl From<RelativeFlowDirection> for FlowDirection {
    fn from(val: RelativeFlowDirection) -> Self {
        FlowDirection::Relative(val)
    }
}

impl From<WritingFlowDirection> for FlowDirection {
    fn from(val: WritingFlowDirection) -> Self {
        FlowDirection::Writing(val)
    }
}

/// Trait for an object that can generate a concrete basis that can be
/// used for math calculations.
///
/// Notably, [`FlowDirection`] implements this.
pub trait CoordinateSystemProvider {
    /// The direction of flow, in cartesian x = left to right, y = top to bottom;
    fn get_axis(&self, parent_hints: &ParentHints) -> Vec2<f32>;

    /// The direction of cross flow, in cartesian x = left to right, y = top to bottom;
    fn get_cross_axis(&self, parent_hints: &ParentHints) -> Vec2<f32>;

    /// The start of the flow, in cartesian x = left to right, y = top to bottom;
    fn get_origin(&self, parent_hints: &ParentHints) -> Vec2<f32>;

    /// A matrix representing the coordinate system. Useful for graphics and math.
    fn get_matrix(&self, parent_hints: &ParentHints) -> Mat3<f32> {
        let wo = self.get_origin(parent_hints);
        let wx = self.get_axis(parent_hints);
        let wy = self.get_cross_axis(parent_hints);

        Mat3::new(wx.x, wx.y, 0.0, wy.x, wy.y, 0.0, wo.x, wo.y, 1.0)
    }

    /// Transforming into cartesian flow direction!
    fn as_cartesian(&self, parent_hints: &ParentHints) -> CartesianFlowDirection;
}

impl CoordinateSystemProvider for FlowDirection {
    fn get_axis(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            FlowDirection::Cartesian(flow) => flow.get_axis(parent_hints),
            FlowDirection::Relative(flow) => flow.get_axis(parent_hints),
            FlowDirection::Writing(flow) => flow.get_axis(parent_hints),
        }
    }

    fn get_cross_axis(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            FlowDirection::Cartesian(flow) => flow.get_cross_axis(parent_hints),
            FlowDirection::Relative(flow) => flow.get_cross_axis(parent_hints),
            FlowDirection::Writing(flow) => flow.get_cross_axis(parent_hints),
        }
    }

    fn get_origin(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            FlowDirection::Cartesian(flow) => flow.get_origin(parent_hints),
            FlowDirection::Relative(flow) => flow.get_origin(parent_hints),
            FlowDirection::Writing(flow) => flow.get_origin(parent_hints),
        }
    }

    fn as_cartesian(&self, parent_hints: &ParentHints) -> CartesianFlowDirection {
        match self {
            FlowDirection::Cartesian(flow) => flow.as_cartesian(parent_hints),
            FlowDirection::Relative(flow) => flow.as_cartesian(parent_hints),
            FlowDirection::Writing(flow) => flow.as_cartesian(parent_hints),
        }
    }
}

impl CoordinateSystemProvider for CartesianFlowDirection {
    fn get_axis(&self, _parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            CartesianFlowDirection::LeftToRight => Vec2::unit_x(),
            CartesianFlowDirection::RightToLeft => -Vec2::unit_x(),
            CartesianFlowDirection::TopToBottom => Vec2::unit_y(),
            CartesianFlowDirection::BottomToTop => -Vec2::unit_y(),
        }
    }

    fn get_cross_axis(&self, _parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            CartesianFlowDirection::LeftToRight => Vec2::unit_y(),
            CartesianFlowDirection::RightToLeft => -Vec2::unit_y(),
            CartesianFlowDirection::TopToBottom => Vec2::unit_x(),
            CartesianFlowDirection::BottomToTop => -Vec2::unit_x(),
        }
    }

    fn get_origin(&self, _parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            CartesianFlowDirection::LeftToRight => Vec2::zero(),
            CartesianFlowDirection::RightToLeft => Vec2::unit_x(),
            CartesianFlowDirection::TopToBottom => Vec2::zero(),
            CartesianFlowDirection::BottomToTop => Vec2::unit_y(),
        }
    }

    fn as_cartesian(&self, _parent_hints: &ParentHints) -> CartesianFlowDirection {
        *self
    }
}

impl CoordinateSystemProvider for RelativeFlowDirection {
    fn get_axis(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            Self::MainAxisForward => parent_hints.current_flow_direction.get_axis(parent_hints),
            Self::MainAxisBackwards => -parent_hints.current_flow_direction.get_axis(parent_hints),
            Self::CrossAxisForward => parent_hints
                .current_cross_flow_direction
                .get_axis(parent_hints),
            Self::CrossAxisBackwards => -parent_hints
                .current_cross_flow_direction
                .get_axis(parent_hints),
        }
    }

    fn get_cross_axis(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            Self::MainAxisForward => parent_hints.current_flow_direction.get_cross_axis(parent_hints),
            Self::MainAxisBackwards => -parent_hints.current_flow_direction.get_cross_axis(parent_hints),
            Self::CrossAxisForward => parent_hints
                .current_cross_flow_direction
                .get_cross_axis(parent_hints),
            Self::CrossAxisBackwards => -parent_hints
                .current_cross_flow_direction
                .get_cross_axis(parent_hints),
        }
    }

    fn get_origin(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            Self::MainAxisForward => parent_hints.current_flow_direction.get_origin(parent_hints),
            Self::MainAxisBackwards => {
                -parent_hints.current_flow_direction.get_origin(parent_hints)
            }
            Self::CrossAxisForward => parent_hints
                .current_cross_flow_direction
                .get_origin(parent_hints),
            Self::CrossAxisBackwards => -parent_hints
                .current_cross_flow_direction
                .get_origin(parent_hints),
        }
    }

    fn as_cartesian(&self, parent_hints: &ParentHints) -> CartesianFlowDirection {
        match self {
            Self::MainAxisForward => parent_hints
                .current_flow_direction
                .as_cartesian(parent_hints),
            Self::MainAxisBackwards => parent_hints
                .current_flow_direction
                .as_cartesian(parent_hints)
                .inverse(),
            Self::CrossAxisForward => parent_hints
                .current_cross_flow_direction
                .as_cartesian(parent_hints),
            Self::CrossAxisBackwards => parent_hints
                .current_cross_flow_direction
                .as_cartesian(parent_hints)
                .inverse(),
        }
    }
}

impl CoordinateSystemProvider for WritingFlowDirection {
    fn get_axis(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            Self::WritingAxisForward => parent_hints
                .current_writing_flow_direction
                .get_axis(parent_hints),
            Self::WritingAxisBackwards => -parent_hints
                .current_writing_flow_direction
                .get_axis(parent_hints),
            Self::WritingCrossAxisForward => parent_hints
                .current_writing_cross_flow_direction
                .get_axis(parent_hints),
            Self::WritingCrossAxisBackwards => -parent_hints
                .current_writing_cross_flow_direction
                .get_axis(parent_hints),
        }
    }

    fn get_cross_axis(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            Self::WritingAxisForward => parent_hints
                .current_writing_flow_direction
                .get_cross_axis(parent_hints),
            Self::WritingAxisBackwards => -parent_hints
                .current_writing_flow_direction
                .get_cross_axis(parent_hints),
            Self::WritingCrossAxisForward => parent_hints
                .current_writing_cross_flow_direction
                .get_cross_axis(parent_hints),
            Self::WritingCrossAxisBackwards => -parent_hints
                .current_writing_cross_flow_direction
                .get_cross_axis(parent_hints),
        }
    }

    fn get_origin(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            Self::WritingAxisForward => parent_hints
                .current_writing_flow_direction
                .get_origin(parent_hints),
            Self::WritingAxisBackwards => -parent_hints
                .current_writing_flow_direction
                .get_origin(parent_hints),
            Self::WritingCrossAxisForward => parent_hints
                .current_writing_cross_flow_direction
                .get_origin(parent_hints),
            Self::WritingCrossAxisBackwards => -parent_hints
                .current_writing_cross_flow_direction
                .get_origin(parent_hints),
        }
    }

    fn as_cartesian(&self, parent_hints: &ParentHints) -> CartesianFlowDirection {
        match self {
            Self::WritingAxisForward => parent_hints
                .current_writing_flow_direction
                .as_cartesian(parent_hints),
            Self::WritingAxisBackwards => parent_hints
                .current_writing_flow_direction
                .as_cartesian(parent_hints)
                .inverse(),
            Self::WritingCrossAxisForward => parent_hints
                .current_writing_cross_flow_direction
                .as_cartesian(parent_hints),
            Self::WritingCrossAxisBackwards => parent_hints
                .current_writing_cross_flow_direction
                .as_cartesian(parent_hints)
                .inverse(),
        }
    }
}
