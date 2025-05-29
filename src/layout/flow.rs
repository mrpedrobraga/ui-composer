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
//! In `en_US`, horizontal containers lay left to right, and vertical components lay top to bottom.
//!
//! There's also the idea of "Axis" and "Cross-Axis." The axis is the direction that the characters flow,
//! the cross-axis is the direction in which the text grows when it wraps.
//!
//! ### [FlowDirection::Relative]
//! This is for composing nested containers. It specifies a flow relative to the parent flow.
//!
//! For example, if the parent from is [WritingFlowDirection::WritingAxisForward] and you use
//! [RelativeFlowDirection::MainAxisBackward], this is the equivalent of using
//! [WritingFlowDirection::WritingAxisBackward].
//!
//! ### [FlowDirection::Cartesian]
//! This should be your last resort — it's absolute flow. Use it only for things that absolutely
//! require things to be shown the same way for everybody. Compasses, drawings, etc.

use crate::prelude::ParentHints;
use arrayvec::ArrayVec;
use cgmath::BaseFloat;
use vek::{Extent2, Rect, Vec2};

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
            FlowDirection::Relative(flow) => flow.get_axes(parent_hints),
            FlowDirection::Writing(flow) => flow.get_axes(parent_hints),
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
    fn get_axes(&self, _parent_hints: &ParentHints) -> Vec2<f32> {
        match self {
            CartesianFlowDirection::LeftToRight => Vec2::unit_x(),
            CartesianFlowDirection::RightToLeft => -Vec2::unit_x(),
            CartesianFlowDirection::TopToBottom => Vec2::unit_y(),
            CartesianFlowDirection::BottomToTop => -Vec2::unit_y(),
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

#[derive(Default)]
struct StackContext {
    offset: Vec2<f32>,
}

/// Stacks several sizes ([`Extent2`]s) one after another.
/// The resulting [`Rect`]s will not be stretched at all.
#[inline(always)]
pub fn stack_rects<I>(sizes: I, gap: f32, vertical: bool) -> impl Iterator<Item = Rect<f32, f32>>
where
    I: Iterator<Item = Extent2<f32>>,
{
    sizes.scan(StackContext::default(), move |cx, item| {
        let rect = Rect::new(cx.offset.x, cx.offset.y, item.w, item.h);

        if vertical {
            cx.offset.y += item.h;
            cx.offset.y += gap;
        } else {
            cx.offset.x += item.w;
            cx.offset.x += gap;
        }

        Some(rect)
    })
}

/// Divides a total number of shares for n elements, where the elements can be biased with a weight, or have a minimum share.
pub fn weighted_division_with_minima<const SIZE: usize, T: BaseFloat + core::iter::Sum>(
    total: T,
    w: &[T; SIZE],
    m: &[T; SIZE],
    tolerance: T,
) -> ArrayVec<T, SIZE> {
    let total_m: T = m.iter().copied().sum();
    let total_w: T = w.iter().copied().sum();

    if total_m >= total || total_w <= T::zero() {
        return ArrayVec::from(*m);
    }

    // Precompute normalized weights
    let normalized_w: Vec<T> = w.iter().map(|&w| w / total_w).collect();

    let equation = |x| {
        total
            - normalized_w
                .iter()
                .zip(m.iter())
                .map(|(nw, m)| m.max(total * *nw * x))
                .sum::<T>()
    };

    let mut lower_bound = T::zero();
    let mut upper_bound = total;

    loop {
        let sample_point = (lower_bound + upper_bound) / T::from(2).unwrap();
        let error = equation(sample_point);

        if error.abs() < tolerance {
            return normalized_w
                .iter()
                .zip(m.iter())
                .map(|(nw, m)| m.max(total * *nw * sample_point))
                .collect();
        }

        if error > T::zero() {
            lower_bound = sample_point;
        } else {
            upper_bound = sample_point;
        }
    }
}
