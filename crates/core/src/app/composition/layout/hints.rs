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
use ui_composer_geometry::flow::CurrentFlow;
use vek::{Extent2, Rect};

/// The parent hints struct.
#[derive(Debug, Clone, Copy)]
pub struct ParentHints {
    pub rect: Rect<f32, f32>,
    pub current_flow: CurrentFlow,
}

/// The child hints struct.
#[derive(Debug, Clone, Copy, Default)]
pub struct ChildHints {
    // TODO: Turn this into a signal or a state.
    pub minimum_size: Extent2<f32>,
}
