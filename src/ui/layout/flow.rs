use crate::prelude::ParentHints;
use vek::Vec2;

#[derive(Debug, Clone, Copy)]
pub enum FlowDirection {
    /// Geometrically, absolutely defined flow.
    Cartesian(CartesianFlowDirection),
    /// Flow aligned relative to the parent flow.
    Aligned(AlignedFlowDirection),
    /// Flow aligned with the writing flow (usually based on locale).
    Writing(WritingFlowDirection),
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

#[derive(Debug, Clone, Copy)]
pub enum AlignedFlowDirection {
    /// Aligned with parent flow. Default: →.
    MainAxisForward,
    /// Opposite of parent flow. Default: ←.
    MainAxisBackwards,
    /// Aligned with parent cross flow. Default: ↓.
    CrossAxisForward,
    /// Opposite of parent cross flow. Default: ↑.
    CrossAxisBackwards,
}

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

impl From<AlignedFlowDirection> for FlowDirection {
    fn from(val: AlignedFlowDirection) -> Self {
        FlowDirection::Aligned(val)
    }
}

impl From<WritingFlowDirection> for FlowDirection {
    fn from(val: WritingFlowDirection) -> Self {
        FlowDirection::Writing(val)
    }
}

pub trait CoordinateSystemProvider {
    /// The direction of flow, in cartesian x = left to right, y = top to bottom;
    fn get_axes(&self, parent_hints: &ParentHints) -> Vec2<f32>;

    /// The start of the flow, in cartesian x = left to right, y = top to bottom;
    fn get_origin(&self, parent_hints: &ParentHints) -> Vec2<f32>;

    /// Transforming into cartesian flow direction!
    fn as_cartesian(&self, parent_hints: &ParentHints) -> CartesianFlowDirection;
}

impl CoordinateSystemProvider for FlowDirection {
    fn get_axes(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            FlowDirection::Cartesian(flow) => flow.get_axes(parent_hints),
            FlowDirection::Aligned(flow) => flow.get_axes(parent_hints),
            FlowDirection::Writing(flow) => flow.get_axes(parent_hints),
        }
    }

    fn get_origin(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            FlowDirection::Cartesian(flow) => flow.get_origin(parent_hints),
            FlowDirection::Aligned(flow) => flow.get_origin(parent_hints),
            FlowDirection::Writing(flow) => flow.get_origin(parent_hints),
        }
    }

    fn as_cartesian(&self, parent_hints: &ParentHints) -> CartesianFlowDirection {
        match self {
            FlowDirection::Cartesian(flow) => flow.as_cartesian(parent_hints),
            FlowDirection::Aligned(flow) => flow.as_cartesian(parent_hints),
            FlowDirection::Writing(flow) => flow.as_cartesian(parent_hints),
        }
    }
}

impl CoordinateSystemProvider for CartesianFlowDirection {
    fn get_axes(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            CartesianFlowDirection::LeftToRight => Vec2::unit_x(),
            CartesianFlowDirection::RightToLeft => -Vec2::unit_x(),
            CartesianFlowDirection::TopToBottom => Vec2::unit_y(),
            CartesianFlowDirection::BottomToTop => -Vec2::unit_y(),
        }
    }

    fn get_origin(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            CartesianFlowDirection::LeftToRight => Vec2::zero(),
            CartesianFlowDirection::RightToLeft => Vec2::unit_x(),
            CartesianFlowDirection::TopToBottom => Vec2::zero(),
            CartesianFlowDirection::BottomToTop => Vec2::unit_y(),
        }
    }

    fn as_cartesian(&self, parent_hints: &ParentHints) -> CartesianFlowDirection {
        *self
    }
}

impl CoordinateSystemProvider for AlignedFlowDirection {
    fn get_axes(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            Self::MainAxisForward => parent_hints.current_flow_direction.get_axes(parent_hints),
            Self::MainAxisBackwards => -parent_hints.current_flow_direction.get_axes(parent_hints),
            Self::CrossAxisForward => parent_hints
                .current_cross_flow_direction
                .get_axes(parent_hints),
            Self::CrossAxisBackwards => -parent_hints
                .current_cross_flow_direction
                .get_axes(parent_hints),
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
    fn get_axes(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            Self::WritingAxisForward => parent_hints
                .current_writing_flow_direction
                .get_axes(parent_hints),
            Self::WritingAxisBackwards => -parent_hints
                .current_writing_flow_direction
                .get_axes(parent_hints),
            Self::WritingCrossAxisForward => parent_hints
                .current_writing_cross_flow_direction
                .get_axes(parent_hints),
            Self::WritingCrossAxisBackwards => -parent_hints
                .current_writing_cross_flow_direction
                .get_axes(parent_hints),
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
