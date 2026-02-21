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
//! # use ui_composer::app::composition::layout::hints::ParentHints;
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

use crate::app::composition::effects::signal::{React, SignalReactExt};
use crate::app::composition::elements::{Blueprint, Environment};
use hints::{ChildHints, ParentHints};
use {
    futures_signals::signal::{Signal, SignalExt},
    vek::{Extent2, Rect},
};

pub mod hints;
mod implementations;

/// The closure-like trait that produces [`Emit`]s.
#[diagnostic::on_unimplemented(
    message = "{Self} is not a [`LayoutItem`] thus can not be used...",
    label = "...in this context...",
    note = "You can use `ResizableItem` to bundle [`Blueprint`]s as UI."
)]
#[must_use = "layout items need to be put in a layout context to be used."]
pub trait LayoutItem: Send {
    type Blueprint;

    /// The size this component prefers to be at. It's usually its minimum size.
    ///
    /// DEPRECATED: Instead of a callback, [Self::lay] will have a "natural size"
    /// reverse [Signal] in its child hints.
    fn get_natural_size(&self) -> Extent2<f32>;

    /// The size this component prefers to be at. It's usually its minimum size.
    ///
    /// DEPRECATED: Instead of a callback, [Self::lay] will have a "minimum size"
    /// reverse [Signal] in its child hints.
    fn get_minimum_size(&self) -> Extent2<f32>;

    /// Renders the content of this layout item with a specific rect.
    fn lay(&mut self, parent_hints: ParentHints) -> Self::Blueprint;

    /// Creates a reactive Element that resizes its content to fit `rect_signal`.
    fn lay_reactive<Sig, Env: Environment>(
        mut self,
        rect_signal: Sig,
        parent_hints: ParentHints,
    ) -> React<impl Signal<Item = Self::Blueprint>, Env>
    where
        Sig: Signal<Item = Rect<f32, f32>> + Send,
        Self: Sized + Send,
        Self::Blueprint: Blueprint<Env>,
    {
        rect_signal
            .map(move |rect| {
                self.lay(ParentHints {
                    rect,
                    ..parent_hints
                })
            })
            .react()
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

pub struct ItemBox<Factory, Item>
where
    Factory: Send + FnMut(ParentHints) -> Item,
{
    hints: ChildHints,
    factory: Factory,
}

impl<Factory, Item> ItemBox<Factory, Item>
where
    Factory: FnMut(ParentHints) -> Item + Send,
{
    pub fn new(factory: Factory) -> Self {
        Self {
            hints: ChildHints::default(),
            factory,
        }
    }
}

impl<F: Send, Item> LayoutItem for ItemBox<F, Item>
where
    F: FnMut(ParentHints) -> Item,
{
    type Blueprint = Item;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.get_minimum_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.hints.minimum_size
    }

    fn lay(&mut self, layout_hints: ParentHints) -> Self::Blueprint {
        (self.factory)(layout_hints)
    }
}

impl<F, Item> Resizable for ItemBox<F, Item>
where
    F: Send + FnMut(ParentHints) -> Item,
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
