//! # Layout
//!
//! This module contains types and utility functions for efficiently calculating layouts.
//!
//! The star of the show here is [`LayoutItem`], a trait that behaves like a closure
//! with some metadata.
//!
//! ## LayoutItem
//!
//! In UI Composer, you draw graphics by producing [`Emit`]. Technically speaking,
//! you can do anything with them — as you can place your graphics anywhere... But if
//! you write a function that produces primitives yourself, you don't have access to
//! internal variables that might be helpful for laying out (window size, hierarchy, theme, etc.),
//! what we call "parent hints";
//!
//! What you can do though is create a higher order function: a function that returns a closure
//! which in turn produces [`Emit`]s, those which can depend on internal data.
//!
//! ```rust
//! # #![allow(non_snake_case)]
//! # use ui_composer::app::composition::reify::Emit;//!
//! # use ui_composer::standard::runners::wgpu::pipeline::text::Text;
//! # use vek::Rgb;
//! # use ui_composer::standard::runners::wgpu::pipeline::UIContext;
//! # use ui_composer::geometry::layout::hints::ParentHints;
//!
//! // Like this.
//! // `text` here is like a "prop" of your component. It's readily available
//! // for you to compose with other standard.
//! fn MyText<F, R>(text: String) -> F
//!     where
//!         F: Fn(ParentHints) -> R,
//!         R: Emit<UIContext> {
//!
//!     // hints is some internal context that's only going to be available later.
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

use crate::app::composition::reify::Emit;
pub use crate::geometry::flow::CoordinateSystem;
use hints::{ChildHints, ParentHints};
use std::marker::PhantomData;
use {
    futures_signals::signal::{Signal, SignalExt},
    vek::{Extent2, Rect},
};
use crate::app::composition::effects::signal::{React, SignalReactExt};
use crate::app::composition::elements::Blueprint;
use crate::geometry::flow;
use crate::state::process::{Pollable, SignalReactItem};

pub mod hints;
pub mod implementations;

/// The closure-like trait that produces [`Emit`]s.
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

    /// Creates a reactive Element that resizes its content to fit `rect_signal`.
    fn bind_reactive<Sig, Env>(
        mut self,
        rect_signal: Sig,
        parent_hints: ParentHints,
    ) -> React<impl Signal<Item = Self::Content>, Env>
    where
        Sig: Signal<Item = Rect<f32, f32>> + Send,
        Self: Sized + Send,
        Self::Content: Blueprint<Env>,
    {
        rect_signal.map(move |rect| {
            self.lay(ParentHints { rect, ..parent_hints })
        }).react()
    }
}

/// A quite interesting auxiliary trait that
/// describes layout items that have size characteristics
/// that might be of interest to the user while they are writing
/// standard in their app.
///
/// You see, it's common for you to write `impl UI` as the return type
/// of standard instead of concrete type... but that might result in loss
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
pub struct ItemBox<Factory, Item, Env>
where
    Factory: Send + FnMut(ParentHints) -> Item,
{
    hints: ChildHints,
    factory: Factory,
    __context: PhantomData<fn() -> Env>,
}

impl<Factory, Item, Env> ItemBox<Factory, Item, Env>
where
    Factory: FnMut(ParentHints) -> Item + Send,
    Item: Blueprint<Env>,
{
    pub fn new(factory: Factory) -> Self {
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
