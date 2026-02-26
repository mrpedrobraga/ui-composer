//! # Flow
//!
//! Containers lay out their elements in a specific kind of way.
//! The direction of flow of a container depends on many factors: app design,
//! but also _culture_ and language. For example, if you are arranging items "in order",
//! some cultures interpret that as "left to right" but others interpret it as "right to left".
//!
//! For accessibility, [Flow] is _semantic_ by default,
//! using ideas of "previous" and "next" instead of "left" and "right".
//!
//! There are three kinds of flow direction:
//!
//! ### [Flow::Writing]
//! This is the one you probably want. If you want to lay items "first" to "last", use this.
//!
//! Containers will lay the items in the order that characters flow in the current locale.
//! In `en_US`, horizontal layout lay left to right, and vertical standard lay top to bottom.
//!
//! There's also the idea of "Axis" and "Cross-Axis." The axis is the direction that the characters flow,
//! the cross-axis is the direction in which the text grows when it wraps.
//!
//! ### [Flow::Relative]
//! This is for composing nested layout. It specifies a flow relative to the parent flow.
//!
//! For example, if the parent from is [WritingFlow::WritingAxisForward] and you use
//! [RelativeFlow::MainAxisBackward], this is the equivalent of using
//! [WritingFlow::WritingAxisBackward].
//!
//! ### [Flow::Cartesian]
//! This should be your last resort — it's absolute flow. Use it only for things that absolutely
//! require things to be shown the same way for everybody. Compasses, drawings, etc.

use vek::{Mat3, Vec2};

pub mod arrangers;

/// The direction that a container spreads its items in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    /// Flow aligned with the writing flow (usually based on locale).
    Writing(WritingFlow),
    /// Flow aligned relative to the parent flow.
    Relative(RelativeFlow),
    /// Geometrically, absolutely defined flow.
    Cartesian(CartesianFlow),
}

/// Specifies the flow direction in a geometric/absolute fashion.
/// Use it for things like graphics or symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CartesianFlow {
    ///→
    LeftToRight,
    ///←
    RightToLeft,
    ///↓
    TopToBottom,
    ///↑
    BottomToTop,
}

impl CartesianFlow {
    // Inverts this flow.
    pub fn invert(&self) -> Self {
        match self {
            Self::LeftToRight => Self::RightToLeft,
            Self::RightToLeft => Self::LeftToRight,
            Self::TopToBottom => Self::BottomToTop,
            Self::BottomToTop => Self::TopToBottom,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            CartesianFlow::LeftToRight | CartesianFlow::RightToLeft => true,
            CartesianFlow::TopToBottom | CartesianFlow::BottomToTop => false,
        }
    }
}

/// Specifies a flow direction relative to the current "flow context."
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativeFlow {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WritingFlow {
    /// Aligned with writing flow. `en-us`: →.
    WritingAxisForward,
    /// Opposite of writing flow. `en-us`: ←.
    WritingAxisBackwards,
    /// Aligned with writing cross flow. `en-us`: ↓.
    WritingCrossAxisForward,
    /// Opposite of writing cross flow. `en-us`: ↑.
    WritingCrossAxisBackwards,
}

impl From<CartesianFlow> for Flow {
    fn from(val: CartesianFlow) -> Self {
        Flow::Cartesian(val)
    }
}

impl From<RelativeFlow> for Flow {
    fn from(val: RelativeFlow) -> Self {
        Flow::Relative(val)
    }
}

impl From<WritingFlow> for Flow {
    fn from(val: WritingFlow) -> Self {
        Flow::Writing(val)
    }
}

/// A set of flow directions that cascades down a layout informing how containers lay down their children.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentFlow {
    pub current_flow_direction: CartesianFlow,
    pub current_cross_flow_direction: CartesianFlow,
    pub current_writing_flow_direction: CartesianFlow,
    pub current_writing_cross_flow_direction: CartesianFlow,
}

impl CurrentFlow {
    /// Returns the geometric axis related to the current writing order.
    pub fn writing_axis(&self) -> Vec2<f32> {
        self.current_writing_flow_direction.get_axis(self)
    }

    /// Returns the geometric axis related to the current paragraph order.
    pub fn writing_cross_axis(&self) -> Vec2<f32> {
        self.current_writing_cross_flow_direction.get_axis(self)
    }

    /// Returns the starting point of the writing context (usually top left or top right).
    pub fn writing_origin(&self) -> Vec2<f32> {
        self.current_writing_flow_direction.get_origin(self)
    }

    /// Returns the starting point of the writing context (usually top left or top right).
    /// Probably the same value as [`Self::writing_origin`], so you should likely use that.
    pub fn writing_cross_origin(&self) -> Vec2<f32> {
        self.current_writing_cross_flow_direction.get_origin(self)
    }

    /// Returns the coordinate system associated with the writing flow.
    pub fn writing_coordinate_system(&self) -> Mat3<f32> {
        self.current_writing_flow_direction.get_matrix(self)
    }
}

/// Trait for an object that can generate a concrete basis that can be
/// used for math calculations.
///
/// Notably, [`Flow`] implements this.
pub trait CoordinateSystem {
    /// The direction of flow, in cartesian x = left to right, y = top to bottom;
    fn get_axis(&self, parent_hints: &CurrentFlow) -> Vec2<f32>;

    /// The direction of cross flow, in cartesian x = left to right, y = top to bottom;
    fn get_cross_axis(&self, parent_hints: &CurrentFlow) -> Vec2<f32>;

    /// The start of the flow, in cartesian x = left to right, y = top to bottom;
    fn get_origin(&self, parent_hints: &CurrentFlow) -> Vec2<f32>;

    /// A matrix representing the coordinate system. Useful for graphics and math.
    fn get_matrix(&self, parent_hints: &CurrentFlow) -> Mat3<f32> {
        let wo = self.get_origin(parent_hints);
        let wx = self.get_axis(parent_hints);
        let wy = self.get_cross_axis(parent_hints);

        Mat3::new(wx.x, wx.y, 0.0, wy.x, wy.y, 0.0, wo.x, wo.y, 1.0)
    }

    /// Transforming into cartesian flow direction!
    fn as_cartesian(&self, current_flow: &CurrentFlow) -> CartesianFlow;
}

impl CoordinateSystem for Flow {
    fn get_axis(&self, parent_hints: &CurrentFlow) -> Vec2<f32> {
        match self {
            Flow::Cartesian(flow) => flow.get_axis(parent_hints),
            Flow::Relative(flow) => flow.get_axis(parent_hints),
            Flow::Writing(flow) => flow.get_axis(parent_hints),
        }
    }

    fn get_cross_axis(&self, parent_hints: &CurrentFlow) -> Vec2<f32> {
        match self {
            Flow::Cartesian(flow) => flow.get_cross_axis(parent_hints),
            Flow::Relative(flow) => flow.get_cross_axis(parent_hints),
            Flow::Writing(flow) => flow.get_cross_axis(parent_hints),
        }
    }

    fn get_origin(&self, parent_hints: &CurrentFlow) -> Vec2<f32> {
        match self {
            Flow::Cartesian(flow) => flow.get_origin(parent_hints),
            Flow::Relative(flow) => flow.get_origin(parent_hints),
            Flow::Writing(flow) => flow.get_origin(parent_hints),
        }
    }

    fn as_cartesian(&self, parent_hints: &CurrentFlow) -> CartesianFlow {
        match self {
            Flow::Cartesian(flow) => flow.as_cartesian(parent_hints),
            Flow::Relative(flow) => flow.as_cartesian(parent_hints),
            Flow::Writing(flow) => flow.as_cartesian(parent_hints),
        }
    }
}

impl CoordinateSystem for CartesianFlow {
    fn get_axis(&self, _parent_hints: &CurrentFlow) -> Vec2<f32> {
        match self {
            CartesianFlow::LeftToRight => Vec2::unit_x(),
            CartesianFlow::RightToLeft => -Vec2::unit_x(),
            CartesianFlow::TopToBottom => Vec2::unit_y(),
            CartesianFlow::BottomToTop => -Vec2::unit_y(),
        }
    }

    fn get_cross_axis(&self, _parent_hints: &CurrentFlow) -> Vec2<f32> {
        match self {
            CartesianFlow::LeftToRight => Vec2::unit_y(),
            CartesianFlow::RightToLeft => -Vec2::unit_y(),
            CartesianFlow::TopToBottom => Vec2::unit_x(),
            CartesianFlow::BottomToTop => -Vec2::unit_x(),
        }
    }

    fn get_origin(&self, _parent_hints: &CurrentFlow) -> Vec2<f32> {
        match self {
            CartesianFlow::LeftToRight => Vec2::zero(),
            CartesianFlow::RightToLeft => Vec2::unit_x(),
            CartesianFlow::TopToBottom => Vec2::zero(),
            CartesianFlow::BottomToTop => Vec2::unit_y(),
        }
    }

    fn as_cartesian(&self, _parent_hints: &CurrentFlow) -> CartesianFlow {
        *self
    }
}

impl CoordinateSystem for RelativeFlow {
    fn get_axis(&self, parent_hints: &CurrentFlow) -> Vec2<f32> {
        match self {
            Self::MainAxisForward => {
                parent_hints.current_flow_direction.get_axis(parent_hints)
            }
            Self::MainAxisBackwards => {
                -parent_hints.current_flow_direction.get_axis(parent_hints)
            }
            Self::CrossAxisForward => parent_hints
                .current_cross_flow_direction
                .get_axis(parent_hints),
            Self::CrossAxisBackwards => -parent_hints
                .current_cross_flow_direction
                .get_axis(parent_hints),
        }
    }

    fn get_cross_axis(&self, parent_hints: &CurrentFlow) -> Vec2<f32> {
        match self {
            Self::MainAxisForward => parent_hints
                .current_flow_direction
                .get_cross_axis(parent_hints),
            Self::MainAxisBackwards => -parent_hints
                .current_flow_direction
                .get_cross_axis(parent_hints),
            Self::CrossAxisForward => parent_hints
                .current_cross_flow_direction
                .get_cross_axis(parent_hints),
            Self::CrossAxisBackwards => -parent_hints
                .current_cross_flow_direction
                .get_cross_axis(parent_hints),
        }
    }

    fn get_origin(&self, parent_hints: &CurrentFlow) -> Vec2<f32> {
        match self {
            Self::MainAxisForward => {
                parent_hints.current_flow_direction.get_origin(parent_hints)
            }
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

    fn as_cartesian(&self, parent_hints: &CurrentFlow) -> CartesianFlow {
        match self {
            Self::MainAxisForward => parent_hints
                .current_flow_direction
                .as_cartesian(parent_hints),
            Self::MainAxisBackwards => parent_hints
                .current_flow_direction
                .as_cartesian(parent_hints)
                .invert(),
            Self::CrossAxisForward => parent_hints
                .current_cross_flow_direction
                .as_cartesian(parent_hints),
            Self::CrossAxisBackwards => parent_hints
                .current_cross_flow_direction
                .as_cartesian(parent_hints)
                .invert(),
        }
    }
}

impl CoordinateSystem for WritingFlow {
    fn get_axis(&self, parent_hints: &CurrentFlow) -> Vec2<f32> {
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

    fn get_cross_axis(&self, parent_hints: &CurrentFlow) -> Vec2<f32> {
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

    fn get_origin(&self, parent_hints: &CurrentFlow) -> Vec2<f32> {
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

    fn as_cartesian(&self, parent_hints: &CurrentFlow) -> CartesianFlow {
        match self {
            Self::WritingAxisForward => parent_hints
                .current_writing_flow_direction
                .as_cartesian(parent_hints),
            Self::WritingAxisBackwards => parent_hints
                .current_writing_flow_direction
                .as_cartesian(parent_hints)
                .invert(),
            Self::WritingCrossAxisForward => parent_hints
                .current_writing_cross_flow_direction
                .as_cartesian(parent_hints),
            Self::WritingCrossAxisBackwards => parent_hints
                .current_writing_cross_flow_direction
                .as_cartesian(parent_hints)
                .invert(),
        }
    }
}
