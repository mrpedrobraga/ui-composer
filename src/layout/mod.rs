//! # Layout
//!
//! This module contains types and utility functions for efficiently calculating layouts.
//!
//! The star of the show here is [`LayoutItem`], a trait that behaves like a closure
//! with some metadata.
//!
//! ## LayoutItem
//!
//! In UI Composer, you draw graphics by producing [`Reifiable`]. Technically speaking,
//! you can do anything with them — as you can place your graphics anywhere... But if
//! you write a function that produces primitives yourself, you don't have access to
//! internal variables that might be helpful for laying out (window size, hierarchy, theme, etc.),
//! what we call "parent hints";
//!
//! What you can do though is create a higher order function: a function that returns a closure
//! which in turn produces [`Reifiable`]s, those which can depend on internal data.
//!
//! ```rust
//! # #![allow(non_snake_case)]
//! # use ui_composer::app::building_blocks::Reifiable;//!
//! # use ui_composer::backends::wgpu::pipeline::text::Text;
//! # use vek::Rgb;
//! # use ui_composer::backends::wgpu::pipeline::UIContext;
//! # use ui_composer::layout::hints::ParentHints;
//!
//! // Like this.
//! // 'text' here is like a "prop" of your component. It's readily available
//! // for you to compose with other components.
//! fn MyText<F, R>(text: String) -> F
//!     where
//!         F: Fn(ParentHints) -> R,
//!         R: Reifiable<UIContext> {
//!
//!     // hints is some internal context that's only gonna be available later.
//!     |hints| {
//!         Text(hints.rect, text, Rgb::white())
//!     }
//! }
//!
//! fn main() {
//!     let string = String::from("Hello, World");
//!     let app = MyText(string);
//! }
//! ```
//!
//! This would work but:
//! 1. It looks mad ugly.
//! 2. We can't attach any metadata to the inner closure like minimum size, etc.
//!
//! It makes sense, instead, to return a struct that houses a closure and also some additional
//! information. Instead of a single concrete struct, the library offers [`LayoutItem`] so that
//! different types can be implemented as you see fit.
//!
//! ## Hints
//!
//! There is some rather subtle intercommunication between "parent" and "children" layout items,
//! those happen through [`ParentHints`] and [`ChildHints`], available in [`hints`]. Check that
//! module out for more detail.
//!
//! ## Flow
//!
//! Some utility functions for calculating layouts are in the [`flow`] module.

use crate::app::building_blocks::Reifiable;
use crate::prelude::process::SignalReactItem;
pub use flow::CoordinateSystemProvider;
use hints::{ChildHints, ParentHints};
use std::marker::PhantomData;
use {
    futures_signals::signal::{Signal, SignalExt},
    vek::{Extent2, Rect},
};

pub mod flow;
pub mod hints;
pub mod implementations;

/// The closure-like trait that produces [`Reifiable`]s.
#[diagnostic::on_unimplemented(
    message = "{Self} is not a `LayoutItem` thus can not be used...",
    label = "...in this context...",
    note = "You can use `ResizableItem` to use graphics/input primitives as layout items."
)]
#[must_use = "layout items need to be put in a layout context to be used."]
pub trait LayoutItem: Send {
    type Content;

    /// The size this component prefers to be at. It's usually its minimum size.
    ///
    /// DEPRECATED: Instead of a callback, [Self::lay] will have a "natural size"
    /// reverse [Signal] in its child hints.
    #[deprecated]
    fn get_natural_size(&self) -> Extent2<f32>;

    /// The size this component prefers to be at. It's usually its minimum size.
    ///
    /// DEPRECATED: Instead of a callback, [Self::lay] will have a "minimum size"
    /// reverse [Signal] in its child hints.
    #[deprecated]
    fn get_minimum_size(&self) -> Extent2<f32>;

    /// Renders the content of this layout item with a specific rect.
    fn lay(&mut self, parent_hints: ParentHints) -> Self::Content;

    /// Creates a reactive node that re-bakes the layout item to fit a container that can change shape.
    fn lay_reactive<S>(
        mut self,
        size_signal: S,
        parent_hints: ParentHints,
    ) -> SignalReactItem<impl Signal<Item = Self::Content>>
    where
        S: Signal<Item = Extent2<f32>> + Send,
        Self: Sized + Send,
    {
        SignalReactItem(size_signal.map(move |new_size| {
            self.lay(ParentHints {
                rect: Rect::new(0.0, 0.0, new_size.w, new_size.h),
                ..parent_hints
            })
        }))
    }
}

/// A quite interesting auxiliary trait that
/// describes layout items that have size characteristics
/// that might be of interest to the user while they are writing
/// components in their app.
///
/// You see, it's common for you to write `impl UI` as the return type
/// of components instead of concrete type... but that might result in loss
/// of functionality for types that have them.
///
/// [`Resizable`] indicates that the item in question can have sizing characteristics
/// edited. Use it like `impl UI + Resizable`.
pub trait Resizable: LayoutItem {
    /// Consumes this [`ItemBox`] and returns a similar one with the minimum size set.
    fn with_minimum_size(self, min_size: Extent2<f32>) -> Self;
}

/// Simple layout item that can be resized by its parent
/// in whatever way the parent sees fit.
pub struct ItemBox<F, A, Context>
where
    F: Send + FnMut(ParentHints) -> A,
{
    hints: ChildHints,
    factory: F,
    __context: PhantomData<fn() -> Context>,
}

impl<F, Items, Res> ItemBox<F, Items, Res>
where
    F: FnMut(ParentHints) -> Items + Send,
    Items: Reifiable<Res>,
{
    /// Creates a new resizable [`LayoutItem`] that redraws using this factory function.
    pub fn new(factory: F) -> Self {
        Self {
            hints: ChildHints::default(),
            factory,
            __context: PhantomData,
        }
    }
}

impl<F: Send, T, Res> LayoutItem for ItemBox<F, T, Res>
where
    F: FnMut(ParentHints) -> T,
{
    type Content = T;

    fn get_natural_size(&self) -> Extent2<f32> {
        #[allow(deprecated)]
        self.get_minimum_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.hints.minimum_size
    }

    fn lay(&mut self, layout_hints: ParentHints) -> Self::Content {
        (self.factory)(layout_hints)
    }
}

impl<F, T, Res> Resizable for ItemBox<F, T, Res>
where
    F: Send + FnMut(ParentHints) -> T,
{
    fn with_minimum_size(self, min_size: Extent2<f32>) -> Self {
        Self {
            hints: ChildHints {
                minimum_size: min_size,
            },
            ..self
        }
    }
}
