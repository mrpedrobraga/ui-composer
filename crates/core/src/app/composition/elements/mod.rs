//! # Blueprints and Elements
//!
//! A [`Blueprint`] describes how to create an [`Element`] in some environment.
//!
//! The [`Blueprint`] trait is parametric (it has the `Environment` parameter)
//! which allows you to implement it multiple times for the same type.
//!
//! For example, if you have a struct `BoxGraphic` you can implement `Blueprint<Desktop> + Blueprint<TUI>`
//! and determine distinct [`Element`]s it creates when you call `Blueprint::make`

use crate::app::composition::algebra::Propagate;
use crate::app::composition::visit::DriveThru;
use ::ui_composer_math::glamour::Size2;
use std::pin::Pin;
use std::task::{Context, Poll};
use ui_composer_input::event::Event;

pub mod implementations;

pub struct DummyEnvironment();

pub trait Blueprint<Env>
where
    Env: Environment,
{
    type Output: Element<Env, Blueprint = Self> + Send;

    fn make(self, env: &Env::BlueprintResources<'_>) -> Self::Output;
}

pub trait Element<Env>: Propagate<Event, bool>
where
    Env: Environment,
{
    type Effect: for<'fx> DriveThru<Env::EffectVisitor<'fx>> + std::fmt::Debug;
    type Blueprint: Blueprint<Env, Output = Self>;

    fn update(&mut self, blueprint: Self::Blueprint, resources: &Env::BlueprintResources<'_>);

    fn effect(&self) -> Self::Effect;

    fn poll_change(
        self: Pin<&mut Self>,
        #[expect(unused)] cx: &mut Context,
        #[expect(unused)] resources: &Env::BlueprintResources<'_>,
    ) -> Poll<Option<()>> {
        Poll::Ready(None)
    }
}

pub trait Environment: Send {
    type BlueprintResources<'make>;
    type EffectVisitor<'fx>;
    const TILE_SIZE: Size2;
}
