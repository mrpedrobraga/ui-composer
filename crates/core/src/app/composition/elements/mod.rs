//! # Blueprints and Elements
//!
//! A [`Blueprint`] describes how to create an [`Element`] in some environment.
//!
//! The [`Blueprint`] trait is parametric (it has the `Environment` parameter)
//! which allows you to implement it multiple times for the same type.
//!
//! For example, if you have a struct `BoxGraphic` you can implement `Blueprint<Desktop> + Blueprint<TUI>`
//! and determine distinct [`Element`]s it creates when you call `Blueprint::make`

use crate::app::composition::algebra::Bubble;
use crate::app::composition::visit::DriveThru;
use crate::app::input::Event;
use std::pin::Pin;
use std::task::{Context, Poll};

pub mod implementations;

pub struct DummyEnvironment();

pub trait Blueprint<Env>
where
    Env: Environment,
{
    type Element: Element<Env>;
    fn make(self, env: &Env) -> Self::Element;
}

pub trait Element<Env: Environment>: Bubble<Event, bool> {
    type Effect<'fx>: DriveThru<Env::EffectVisitor<'fx>>
    where
        Self: 'fx;

    fn effect(&self) -> Self::Effect<'_>;

    fn poll(
        self: Pin<&mut Self>,
        #[expect(unused)] cx: &mut Context,
        #[expect(unused)] env: &Env,
    ) -> Poll<Option<()>> {
        Poll::Ready(None)
    }
}

pub trait Environment {
    type EffectVisitor<'fx>;
}
