use super::super::elements::{Blueprint, Element};
use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[pin_project]
#[must_use = "ReactOnce does nothing unless polled"]
pub struct ReactOnce<Fut, Env>
where
    Fut: Future,
    Fut::Output: Blueprint<Env>,
{
    #[pin]
    future: Fut,
    element: Option<<Fut::Output as Blueprint<Env>>::Element>,
}

pub trait FutureExt: Future {
    fn into_signal<Env>(self) -> ReactOnce<Self, Env>
    where
        Self: Sized,
        <Self as Future>::Output: Blueprint<Env>;
}
impl<Fut> FutureExt for Fut
where
    Fut: Future,
{
    fn into_signal<Env>(self) -> ReactOnce<Self, Env>
    where
        Self: Sized,
        <Self as Future>::Output: Blueprint<Env>,
    {
        ReactOnce {
            future: self,
            element: None,
        }
    }
}

impl<Fut, Env> Blueprint<Env> for ReactOnce<Fut, Env>
where
    Fut: Future<Output: Blueprint<Env>>,
{
    type Element = Self;

    fn make(self, _: &Env) -> Self::Element {
        self
    }
}

impl<Fut, Env> Element<Env> for ReactOnce<Fut, Env>
where
    Fut: Future<Output: Blueprint<Env>>,
{
    type Effect = Option<
        <<<Fut as Future>::Output as Blueprint<Env>>::Element as Element<
            Env,
        >>::Effect,
    >;

    fn effect(&self) -> Self::Effect {
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
