use glamour::Rect;

use crate::runner::{ElementEffectHandler, InitializationResources};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub mod effects;
pub mod implementations;

/// Trait for a piece of the UI that will update as the app runs.
pub trait Ui {
    type Blueprint: Blueprint;

    fn plan(&mut self, parent_hints: ParentHints, resources: &InitializationResources);

    fn effect(&self) -> <<Self::Blueprint as Blueprint>::Output as Element>::Effect;

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &InitializationResources,
        parent_hints: ParentHints,
    ) -> Poll<Option<()>>;
}

#[derive(Clone)]
pub struct ParentHints {
    pub rect: Rect,
}

/// Describes a section of the app.
///
/// ## Lightweight
/// Blueprints may be created again and again pretty much every frame,
/// so they should be cheap to create.
///
/// [Element]s, on the other hand, are not cheap and can hold onto
/// heavy runtime resources.
pub trait Blueprint {
    type Output: Element<Blueprint = Self>;

    fn make(self, resources: &InitializationResources) -> Self::Output;
}

/// A section of the running app.
pub trait Element {
    type Effect: ElementEffect;
    type Blueprint: Blueprint<Output = Self>;

    /// Updates this element given a blueprint.
    fn update(&mut self, blueprint: Self::Blueprint, resources: &InitializationResources);

    fn effect(&self) -> Self::Effect;

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>>;
}

/// An effect that the element can have over the application :-)
pub trait ElementEffect {
    fn apply(&self, consumer: &mut ElementEffectHandler);
}

fn max(a: Poll<Option<()>>, b: Poll<Option<()>>) -> Poll<Option<()>> {
    use std::task::Poll::*;

    match (a, b) {
        (Ready(Some(())), _) | (_, Ready(Some(()))) => Ready(Some(())),
        (Pending, _) | (_, Pending) => Pending,
        _ => Ready(None),
    }
}
