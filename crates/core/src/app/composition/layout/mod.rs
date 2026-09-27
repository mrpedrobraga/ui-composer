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

use super::elements::{Blueprint, Element, Environment};
use hints::{ChildHints, ParentHints};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use ui_composer_math::prelude::Size2;

pub mod hints;
mod implementations;

/// The closure-like trait that produces [`Emit`]s.
#[diagnostic::on_unimplemented(
    message = "{Self} is not [Ui] and thus can not be used...",
    label = "...in this context...",
    note = "You can use [Canvas] to bundle [Blueprint]s as [Ui]!."
)]
#[must_use = "Ui needs to be given to a context (such as a window) to do anything."]
pub trait Ui<Env>: Send
where
    Env: Environment,
{
    type Blueprint: Blueprint<Env>;

    fn prepare(&mut self, expected_parent_hints: ParentHints) -> ChildHints;

    fn place(&mut self, parent_hints: ParentHints, resources: &Env::BlueprintResources<'_>);

    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect;

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &Env::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> Poll<Option<()>>;

    fn boxed(self) -> Box<dyn Ui<Env, Blueprint = Self::Blueprint>>
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
pub trait Resizable<Env>: Ui<Env>
where
    Env: Environment,
{
    /// Consumes this [`ItemBox`] and returns a similar one with the minimum size set.
    fn with_minimum_size(self, min_size: Size2) -> Self;
}

#[pin_project::pin_project]
pub struct Canvas<Env, B, F>
where
    B: Blueprint<Env>,
    F: Send + FnMut(ParentHints) -> B,
    Env: Environment,
{
    #[pin]
    elements: Option<B::Output>,
    maker: F,
    hints: ChildHints,
}

impl<Env, B, F> Canvas<Env, B, F>
where
    B: Blueprint<Env>,
    F: Send + FnMut(ParentHints) -> B,
    Env: Environment,
{
    pub fn new(maker: F) -> Self {
        Self {
            hints: ChildHints::default(),
            elements: None,
            maker,
        }
    }
}

impl<Env, B, F> Ui<Env> for Canvas<Env, B, F>
where
    B: Blueprint<Env, Output: Send>,
    F: Send + FnMut(ParentHints) -> B,
    Env: Environment,
{
    type Blueprint = B;

    fn prepare(&mut self, _: ParentHints) -> ChildHints {
        self.hints
    }

    fn place(&mut self, parent_hints: ParentHints, resources: &Env::BlueprintResources<'_>) {
        // (self.maker)(layout_hints)
        let new_blueprint = (self.maker)(parent_hints);
        if let Some(elements) = &mut self.elements {
            elements.update(new_blueprint, resources);
        } else {
            self.elements = Some(new_blueprint.make(resources))
        }
    }

    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect {
        self.elements.as_ref().unwrap().effect()
    }

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &Env::BlueprintResources<'_>,
        _: ParentHints,
    ) -> Poll<Option<()>> {
        let this = self.project();
        this.elements.poll_change(cx, resources)
    }
}

impl<Env, B, F> Resizable<Env> for Canvas<Env, B, F>
where
    B: Blueprint<Env, Output: Send>,
    F: Send + FnMut(ParentHints) -> B,
    Env: Environment,
{
    fn with_minimum_size(self, min_size: Size2) -> Self {
        Self {
            hints: ChildHints {
                minimum_size: min_size,
            },
            ..self
        }
    }
}
