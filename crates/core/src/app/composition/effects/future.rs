//! # Effects/Future
//! 
//! A `Future<Output = T>` is "a `T` that will appear later."
//! In classic functorial fashion, if `T` is an UI element with an effect,
//! `Future<Output = T>` is _also_ an element.
//! 
//! [`ReactOnce`] wraps the future so it can hold onto the `T` it resolves to.

use crate::app::composition::algebra::Bubble;
use crate::app::composition::elements::Environment;
use ui_composer_input::event::Event;
use super::super::elements::{Blueprint, Element};
use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Wraps a future, holding onto the elements it produces.
/// 
/// Notably implements `Blueprint` and `Element.`
#[pin_project]
#[must_use = "ReactOnce does nothing unless polled"]
pub struct ReactOnce<Fut, Env: Environment>
where
    Fut: Future,
    Fut::Output: Blueprint<Env>,
{
    #[pin]
    future: Fut,
    element: Option<<Fut::Output as Blueprint<Env>>::Element>,
}

impl<Fut, Env: Environment> Blueprint<Env> for ReactOnce<Fut, Env>
where
    Fut: Future<Output: Blueprint<Env>>,
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
    fn bubble(&mut self, cx: &mut Event) -> bool {
        self.element
            .as_mut()
            .map(|e| e.bubble(cx))
            .unwrap_or_default()
    }
}

impl<Fut, Env: Environment> Element<Env> for ReactOnce<Fut, Env>
where
    Fut: Future<Output: Blueprint<Env>>,
{
    type Effect<'fx>
        = Option<
        <<<Fut as Future>::Output as Blueprint<Env>>::Element as Element<
            Env,
        >>::Effect<'fx>,
    >
    where
        Fut: 'fx,
        Env: 'fx;

    fn effect(&self) -> Self::Effect<'_> {
        self.element.as_ref().map(|e| e.effect())
    }

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context,
        env: &Env::BlueprintResources<'_>,
    ) -> Poll<Option<()>> {
        let this = self.project();

        if let Some(element) = this.element {
            // SAFETY: we can pin element here because `self` is pinned.
            return unsafe { Pin::new_unchecked(element) }.poll(cx, env);
        }

        *this.element = None;

        // SAFETY: Because the future is pinned in this struct, its captures are stable.
        match this.future.poll(cx) {
            Poll::Ready(blueprint) => {
                let mut element = blueprint.make(env);

                // Wake up the element.
                let _ =
                    unsafe { Pin::new_unchecked(&mut element) }.poll(cx, env);
                *this.element = Some(element);

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
{
    type Output = ReactOnce<Fut, Env>;

    fn into_blueprint(self) -> Self::Output {
        ReactOnce {
            future: self,
            element: None,
        }
    }
}
