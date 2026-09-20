//! # Effects/Future
//!
//! A `Future<Output = T>` is "a `T` that will appear later."
//! In classic functorial fashion, if `T` is an UI element with an effect,
//! `Future<Output = T>` is _also_ an element.
//!
//! [`ReactOnce`] wraps the future so it can hold onto the `T` it resolves to.

use super::super::elements::{Blueprint, Element};
use crate::app::composition::algebra::Bubble;
use crate::app::composition::elements::Environment;
use futures_signals::signal::Mutable;
use pin_project::pin_project;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use ui_composer_input::event::Event;

/// Wraps a future, holding onto the elements it produces.
///
/// Notably implements `Blueprint` and `Element.`
#[pin_project]
#[must_use = "ReactOnce does nothing unless polled"]
pub struct ReactOnce<Fut, Env>
where
    Fut: Future,
    Fut::Output: Blueprint<Env>,
    Env: Environment,
{
    #[pin]
    future: Fut,
    element: Mutable<Option<<Fut::Output as Blueprint<Env>>::Element>>,
}

impl<Fut, Env> Clone for ReactOnce<Fut, Env>
where
    Fut: Future + Clone,
    Fut::Output: Blueprint<Env>,
    <Fut::Output as Blueprint<Env>>::Element: Clone + Debug,
    Env: Environment,
{
    fn clone(&self) -> Self {
        Self {
            future: self.future.clone(),
            element: self.element.clone(),
        }
    }
}

impl<Fut, Env> std::fmt::Debug for ReactOnce<Fut, Env>
where
    Fut: Future + Clone,
    Fut::Output: Blueprint<Env>,
    <Fut::Output as Blueprint<Env>>::Element: Clone + Debug,
    Env: Environment,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReactOnce")
            .field("element", &self.element)
            .finish()
    }
}

impl<Fut, Env: Environment> Blueprint<Env> for ReactOnce<Fut, Env>
where
    Fut: Future<Output: Blueprint<Env>>,
    <<<Fut as futures::Future>::Output as Blueprint<Env>>::Element as Element<Env>>::Effect:
        std::clone::Clone,
{
    type Element = Self;

    fn make(self, _: &Env::BlueprintResources<'_>) -> Self::Element {
        self
    }
}

impl<Fut, Env: Environment> Bubble<Event, bool> for ReactOnce<Fut, Env>
where
    Fut: Future<Output: Blueprint<Env>>,
{
    async fn bubble(&mut self, cx: &mut Event) -> bool {
        let mut guard = self.element.lock_mut();
        if let Some(e) = &mut *guard {
            e.bubble(cx).await
        } else {
            false
        }
    }
}

impl<Fut, Env: Environment> Element<Env> for ReactOnce<Fut, Env>
where
    Fut: Future<Output: Blueprint<Env>>,
    <<<Fut as Future>::Output as Blueprint<Env>>::Element as Element<Env>>::Effect: Clone,
{
    type Effect =
        Option<<<<Fut as Future>::Output as Blueprint<Env>>::Element as Element<Env>>::Effect>;

    fn effect(&self) -> Self::Effect {
        let guard = self.element.lock_ref();
        guard.as_ref().map(|e| e.effect().clone())
    }

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context,
        env: &Env::BlueprintResources<'_>,
    ) -> Poll<Option<()>> {
        let this = self.project();

        if let Some(element) = this.element.lock_mut().as_mut() {
            // SAFETY: we can pin element here because `self` is pinned.
            let poll = unsafe { Pin::new_unchecked(element) }.poll(cx, env);
            return poll;
        }

        // *this.element = None;

        // SAFETY: Because the future is pinned in this struct, its captures are stable.
        match this.future.poll(cx) {
            Poll::Ready(blueprint) => {
                let mut element = blueprint.make(env);

                // Wake up the element.
                let _ = unsafe { Pin::new_unchecked(&mut element) }.poll(cx, env);
                println!("Putting element inside.");
                this.element.set(Some(element));

                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Handy trait for transforming a Future into a `Blueprint` for an environment.
///
/// ```no_run
/// let my_future = async { Text("Hello, World!") };
///
/// // Currently, you can't do this, because `Blueprint` isn't implemented for `Future`.
/// let bp: Blueprint<Env> = my_future;
/// // Do this instead:
/// let bp: Blueprint<Env> = my_future.into_blueprint();
/// ```
///
/// We can't implement `Blueprint` for all futures without problems,
/// so we need to a type this crate owns.
///
/// The automatic implementation that produces a [`ReactOnce`] without a held item.
///
/// This will no longer be a kink when `min_specialization` gets stabilized.
/// When it does, you'll be able to directly use a future directly wherever a `Blueprint` is required.
pub trait IntoBlueprint<Env: Environment> {
    type Output: Blueprint<Env>;

    fn into_blueprint(self) -> Self::Output;
}

impl<Fut, Env> IntoBlueprint<Env> for Fut
where
    Fut: Future,
    Env: Environment,
    <Fut as futures::Future>::Output: Blueprint<Env>,
    <<<Fut as futures::Future>::Output as Blueprint<Env>>::Element as Element<Env>>::Effect:
        std::clone::Clone,
{
    type Output = ReactOnce<Fut, Env>;

    fn into_blueprint(self) -> Self::Output {
        ReactOnce {
            future: self,
            element: Mutable::new(None),
        }
    }
}
