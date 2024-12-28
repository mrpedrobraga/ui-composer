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

pub trait CoordinateSystemProvider {
    /// The direction of flow, in cartesian x = left to right, y = top to bottom;
    fn get_axis(&self, parent_hints: &ParentHints) -> Vec2<f32>;

    /// The start of the flow, in cartesian x = left to right, y = top to bottom;
    fn get_origin(&self, parent_hints: &ParentHints) -> Vec2<f32>;
}

impl CoordinateSystemProvider for FlowDirection {
    fn get_axis(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            FlowDirection::Cartesian(flow) => flow.get_axis(parent_hints),
            FlowDirection::Aligned(flow) => flow.get_axis(parent_hints),
            FlowDirection::Writing(flow) => flow.get_axis(parent_hints),
        }
    }

    fn get_origin(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            FlowDirection::Cartesian(flow) => flow.get_origin(parent_hints),
            FlowDirection::Aligned(flow) => flow.get_origin(parent_hints),
            FlowDirection::Writing(flow) => flow.get_origin(parent_hints),
        }
    }
}

impl CoordinateSystemProvider for CartesianFlowDirection {
    fn get_axis(&self, parent_hints: &ParentHints) -> Vec2<f32> {
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
}

impl CoordinateSystemProvider for AlignedFlowDirection {
    fn get_axis(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            AlignedFlowDirection::MainAxisForward => {
                parent_hints.current_flow_direction.get_axis(parent_hints)
            }
            AlignedFlowDirection::MainAxisBackwards => {
                -parent_hints.current_flow_direction.get_axis(parent_hints)
            }
            AlignedFlowDirection::CrossAxisForward => parent_hints
                .current_cross_flow_direction
                .get_axis(parent_hints),
            AlignedFlowDirection::CrossAxisBackwards => -parent_hints
                .current_cross_flow_direction
                .get_axis(parent_hints),
        }
    }

    fn get_origin(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            AlignedFlowDirection::MainAxisForward => {
                parent_hints.current_flow_direction.get_origin(parent_hints)
            }
            AlignedFlowDirection::MainAxisBackwards => {
                -parent_hints.current_flow_direction.get_origin(parent_hints)
            }
            AlignedFlowDirection::CrossAxisForward => parent_hints
                .current_cross_flow_direction
                .get_origin(parent_hints),
            AlignedFlowDirection::CrossAxisBackwards => -parent_hints
                .current_cross_flow_direction
                .get_origin(parent_hints),
        }
    }
}

impl CoordinateSystemProvider for WritingFlowDirection {
    fn get_axis(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            WritingFlowDirection::WritingAxisForward => parent_hints
                .current_writing_flow_direction
                .get_axis(parent_hints),
            WritingFlowDirection::WritingAxisBackwards => -parent_hints
                .current_writing_flow_direction
                .get_axis(parent_hints),
            WritingFlowDirection::WritingCrossAxisForward => parent_hints
                .current_writing_cross_flow_direction
                .get_axis(parent_hints),
            WritingFlowDirection::WritingCrossAxisBackwards => -parent_hints
                .current_writing_cross_flow_direction
                .get_axis(parent_hints),
        }
    }

    fn get_origin(&self, parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            WritingFlowDirection::WritingAxisForward => parent_hints
                .current_writing_flow_direction
                .get_origin(parent_hints),
            WritingFlowDirection::WritingAxisBackwards => -parent_hints
                .current_writing_flow_direction
                .get_origin(parent_hints),
            WritingFlowDirection::WritingCrossAxisForward => parent_hints
                .current_writing_cross_flow_direction
                .get_origin(parent_hints),
            WritingFlowDirection::WritingCrossAxisBackwards => -parent_hints
                .current_writing_cross_flow_direction
                .get_origin(parent_hints),
        }
    }
}
