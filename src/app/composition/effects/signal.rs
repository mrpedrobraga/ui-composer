use std::pin::Pin;
use std::task::{Context, Poll};
use futures_signals::signal::Signal;
use pin_project::pin_project;
use crate::app::composition::elements::{Blueprint, Element};

#[pin_project]
#[must_use = "React does nothing unless polled"]
pub struct React<Sig, Env> where Sig: Signal, Sig::Item: Blueprint<Env> {
    #[pin]
    signal: Sig,
    element: Option<<Sig::Item as Blueprint<Env>>::Element>,
}

pub trait SignalReactExt: Signal {
    fn react<Env>(self) -> React<Self, Env>
    where
        Self: Sized,
        <Self as Signal>::Item: Blueprint<Env>;
}
impl<Sig> SignalReactExt for Sig where Sig: Signal {
    fn react<Env>(self) -> React<Self, Env>
    where
        Self: Sized,
        <Self as Signal>::Item: Blueprint<Env>
    {
        React { signal: self, element: None }
    }
}

impl<Sig, Env> Blueprint<Env> for React<Sig, Env>
where
    Sig: Signal<Item: Blueprint<Env>>
{
    type Element = Self;

    fn make(self, _: &Env) -> Self::Element {
        self
    }
}

impl<Sig, Env> Element<Env> for React<Sig, Env>
where
    Sig: Signal<Item: Blueprint<Env>> {
    type Effect = Option<<<<Sig as Signal>::Item as Blueprint<Env>>::Element as Element<Env>>::Effect>;

    fn effect(&self) -> Self::Effect {
        self.element.as_ref().map(|e| e.effect())
    }

    fn poll(self: Pin<&mut Self>, cx: &mut Context, env: &Env) -> Poll<Option<()>> {
        let this = self.project();

        if let Some(element) = this.element {
            // SAFETY: we can pin element here because `self` is pinned
            // and will remain pinned until the next `poll`, by which `element`
            // will be dropped and replaced.
            return unsafe { Pin::new_unchecked(element) }.poll(cx, env);
        }

        *this.element = None;

        // SAFETY: Because the signal is pinned in this struct, its captures are stable.
        match this.signal.poll_change(cx) {
            Poll::Ready(Some(blueprint)) => {
                let mut element = blueprint.make(env);

                // Wake up the element.
                let _ = unsafe { Pin::new_unchecked(&mut element) }.poll(cx, env);
                *this.element = Some(element);

                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
        }
    }
}