//! # Blueprints and Elements
//!
//! A [`Blueprint`] describes how to create an [`Element`] in some environment.
//!
//! The [`Blueprint`] trait is parametric (it has the `Environment` parameter)
//! which allows you to implement it multiple times for the same type.
//! 
//! For example, if you have a struct `BoxGraphic` you can implement `Blueprint<Desktop> + Blueprint<TUI>`
//! and determine distinct [`Element`]s it creates when you call `Blueprint::make`

use std::pin::Pin;
use std::task::{Context, Poll};
use downcast_rs::{impl_downcast, Downcast};
use crate::app::composition::effects::ElementEffect;

pub mod implementations;

pub struct DummyEnvironment();

pub trait Blueprint<Environment> {
    type Element: Element<Environment>;
    fn make(self, env: &Environment) -> Self::Element;
}

pub trait Element<Environment> {
    type Effect: ElementEffect;

    fn effect(&self) -> Self::Effect;

    fn poll(
        self: Pin<&mut Self>,
        #[expect(unused)] cx: &mut Context,
        #[expect(unused)] env: &Environment,
    ) -> Poll<Option<()>> {
        Poll::Ready(None)
    }
}
