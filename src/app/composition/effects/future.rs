use super::super::elements::{Blueprint, Element};
use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

#[pin_project(project = HoldFutureProj)]
#[must_use = "HoldFutures do nothing unless polled"]
pub enum ReactOnce<Fut, Env>
where
    Fut: Future<Output: Blueprint<Env>>,
{
    Pending(#[pin] Fut),
    Done(#[pin] <Fut::Output as Blueprint<Env>>::Element),
}

pub trait React: Future {
    fn react<Env>(self) -> ReactOnce<Self, Env>
    where
        Self: Sized,
        <Self as Future>::Output: Blueprint<Env>;
}
impl<Fut> React for Fut where Fut: Future {
    fn react<Env>(self) -> ReactOnce<Self, Env>
    where
        Self: Sized,
        <Self as Future>::Output: Blueprint<Env>
    {
        ReactOnce::Pending(self)
    }
}

impl<Fut, Env> Blueprint<Env> for ReactOnce<Fut, Env>
where
    Fut: Future<Output: Blueprint<Env>>, {
    type Element = Self;

    fn make(self, _: &Env) -> Self::Element {
        self
    }
}

impl<Fut, Env> Element<Env> for ReactOnce<Fut, Env>
where
    Fut: Future<Output: Blueprint<Env>>,
{
    type Effect =
        Option<<<<Fut as Future>::Output as Blueprint<Env>>::Element as Element<Env>>::Effect>;

    fn effect(&self) -> Self::Effect {
        match self {
            ReactOnce::Pending(_) => None,
            ReactOnce::Done(element) => Some(element.effect()),
        }
    }

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context, env: &Env) -> Poll<Option<()>> {
        let this = self.as_mut().project();

        match this {
            HoldFutureProj::Pending(fut) => match fut.poll(cx) {
                Poll::Ready(blueprint) => {
                    let mut element = blueprint.make(env);
                    let _ = unsafe { Pin::new_unchecked(&mut element) }.poll(cx, env);
                    self.set(ReactOnce::Done(element));
                    Poll::Ready(Some(()))
                }
                Poll::Pending => Poll::Pending,
            },
            HoldFutureProj::Done(item) => item.poll(cx, env),
        }
    }
}
