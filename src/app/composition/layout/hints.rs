//! # Hints
//!
//! When making your apps, you'll likely be putting standard together.
//!
//! Thing is, you'll be combining standard like Buttons, Labels, etc., with layout,
//! and, well, how is the Button supposed to know what size to be, given that this information
//! is only available at runtime and inside the layout engine?
//!
//! To solve that, as you've read in [`super`], there exists the trait [`LayoutItem`],
//! which is like a closure that produces [`Reifiable`] based on some internal context.
//!
//! [`ParentHints`] is that context.
//!
//! By default, layout stuff is passed through it, though there are plans of allowing the user
//! to pass anything they want (by introducing a generic parameter). This isn't available yet
//! because, as of now, it would be incredibly annoying to use. Man, generics are so hard to keep tidy.
//!
//! [`ChildHints`] is a bundle of context a child might reply to the parent with. It might
//! contain information like minimum size, natural size, etc., which are useful for layout calculations.
//!
//! The usual calculation order inside a Container works like this:
//!
//! 1. Get the children's child hints, likely in order;
//! 2. Perform layout calculations;
//! 3. Generate parent hints, likely in order, while calling [`LayoutItem::lay`] on them.;
//!
use crate::app::composition::layout::CoordinateSystem;
use crate::geometry::flow::CartesianFlow;
use vek::{Extent2, Mat3, Rect, Vec2};

/// The parent hints struct.
#[derive(Debug, Clone, Copy)]
pub struct ParentHints {
    pub rect: Rect<f32, f32>,
    pub current_flow_direction: CartesianFlow,
    pub current_cross_flow_direction: CartesianFlow,
    pub current_writing_flow_direction: CartesianFlow,
    pub current_writing_cross_flow_direction: CartesianFlow,
}

/// The child hints struct.
#[derive(Debug, Clone, Copy, Default)]
pub struct ChildHints {
    // TODO: Turn this into a signal or a state.
    pub minimum_size: Extent2<f32>,
}

impl ParentHints {
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
