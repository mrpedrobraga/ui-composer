use crate::app::composition::algebra::Bubble;
use crate::app::composition::elements::Environment;
use ui_composer_input::event::Event;

use super::super::elements::{Blueprint, Element};
use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

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

    fn make(self, _: &Env) -> Self::Element {
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
        env: &Env,
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

/*
    This is necessary while we don't have `min_specialization`.

    We can't implement `Blueprint` for all futures without problems,
    so we need to a type this crate owns.
*/

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
