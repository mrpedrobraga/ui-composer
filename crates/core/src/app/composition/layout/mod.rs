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

use crate::app::composition::effects::signal::{IntoBlueprint as _, React};
use crate::app::composition::elements::{Blueprint, Environment};
use futures_signals::signal::{Signal, SignalExt};
use hints::{ChildHints, ParentHints};
use ui_composer_math::prelude::{Rect, Size2};

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

    /// Prepares the item for laying out.
    fn prepare(&mut self, expected_parent_hints: ParentHints) -> ChildHints;

    /// Renders the content of this layout item with a specific rect.
    fn place(
        &mut self,
        // TODO: Reflect on whether it's necessary to pass any context when calling `place`.
        parent_hints: ParentHints,
    ) -> Self::Blueprint;

    /// Creates a reactive Element that resizes its content to fit `rect_signal`.
    fn place_reactive<Sig, Env: Environment>(
        mut self,
        rect_signal: Sig,
        parent_hints: ParentHints,
    ) -> React<impl Signal<Item = Self::Blueprint>, Env>
    where
        Sig: Signal<Item = Rect> + Send,
        Self: Sized + Send,
        Self::Blueprint: Blueprint<Env>,
    {
        rect_signal
            .map(move |rect| {
                self.place(ParentHints {
                    rect,
                    ..parent_hints
                })
            })
            .into_blueprint()
    }

    /// Erases the type of the layout item, allocating it on the heap,
    /// while remembering the type of `Blueprint` the item generates.
    ///
    /// This is useful wherever you need to pass two or more items of the same concrete type,
    /// but would like to pass different UI... for example, you can call `boxed`
    /// to return different UI from `match` arms.
    ///
    /// This obviously adds some indirection as well as some heap allocation
    /// so make of that what you will.
    fn boxed(self) -> Box<dyn LayoutItem<Blueprint = Self::Blueprint>>
    where
        Self: std::marker::Sized + 'static,
    {
        Box::new(self)
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
    fn with_minimum_size(self, min_size: Size2) -> Self;
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

    fn prepare(&mut self, _: ParentHints) -> ChildHints {
        self.hints
    }

    fn place(&mut self, layout_hints: ParentHints) -> Self::Blueprint {
        (self.factory)(layout_hints)
    }
}

impl<F, Item> Resizable for ItemBox<F, Item>
where
    F: Send + FnMut(ParentHints) -> Item,
{
    fn with_minimum_size(self, min_size: Size2) -> Self {
        Self {
            hints: ChildHints {
                minimum_size: min_size,
                natural_size: min_size,
            },
            ..self
        }
    }
}

// ---

pub struct ItemBox2<Capture, Factory, Item>
where
    Factory: Send + FnMut(&mut Capture, ParentHints) -> Item,
{
    capture: Capture,
    hints: ChildHints,
    factory: Factory,
}

impl<Capture, Factory, Item> ItemBox2<Capture, Factory, Item>
where
    Factory: FnMut(&mut Capture, ParentHints) -> Item + Send,
{
    pub fn new(capture: Capture, factory: Factory) -> Self {
        Self {
            capture,
            hints: ChildHints::default(),
            factory,
        }
    }
}

impl<Capture, F: Send, Item> LayoutItem for ItemBox2<Capture, F, Item>
where
    F: FnMut(&mut Capture, ParentHints) -> Item,
    Capture: std::marker::Send,
{
    type Blueprint = Item;

    fn prepare(&mut self, _: ParentHints) -> ChildHints {
        self.hints
    }

    fn place(&mut self, layout_hints: ParentHints) -> Self::Blueprint {
        (self.factory)(&mut self.capture, layout_hints)
    }
}

impl<Capture, F, Item> Resizable for ItemBox2<Capture, F, Item>
where
    F: Send + FnMut(&mut Capture, ParentHints) -> Item,
    Capture: std::marker::Send,
{
    fn with_minimum_size(self, min_size: Size2) -> Self {
        Self {
            hints: ChildHints {
                minimum_size: min_size,
                ..self.hints
            },
            ..self
        }
    }
}
