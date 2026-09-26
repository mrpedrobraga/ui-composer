use crate::runner::ElementEffectHandler;
use std::{
    cell::OnceCell,
    pin::Pin,
    task::{Context, Poll},
};

pub mod effects;

pub trait Element {
    type Effect: ElementEffect;

    fn effect(&self) -> Self::Effect;

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>>;
}

pub trait ElementEffect {
    fn apply(&self, consumer: &mut ElementEffectHandler);
}

impl<A, B> Element for (A, B)
where
    A: Element,
    B: Element,
{
    type Effect = (A::Effect, B::Effect);

    fn effect(&self) -> Self::Effect {
        (self.0.effect(), self.1.effect())
    }

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        let (pinned_a, pinned_b) = {
            let mut_ref = unsafe { self.get_unchecked_mut() };
            let (a, b) = mut_ref;

            let a = unsafe { Pin::new_unchecked(a) };
            let b = unsafe { Pin::new_unchecked(b) };

            (a, b)
        };

        let poll_a = pinned_a.poll_change(cx);
        let poll_b = pinned_b.poll_change(cx);

        combine_polls(poll_a, poll_b)
    }
}

impl<A, B> ElementEffect for (A, B)
where
    A: ElementEffect,
    B: ElementEffect,
{
    fn apply(&self, consumer: &mut ElementEffectHandler) {
        self.0.apply(consumer);
        self.1.apply(consumer);
    }
}

impl<A> ElementEffect for Option<A>
where
    A: ElementEffect,
{
    fn apply(&self, consumer: &mut ElementEffectHandler) {
        if let Some(element) = &self {
            element.apply(consumer);
        }
    }
}

#[pin_project::pin_project(project = DelayedProj)]
pub struct DelayedUntil<E, F> {
    #[pin]
    element: E,
    ready: bool,
    #[pin]
    deadline: F,
}

impl<E, F> DelayedUntil<E, F> {
    pub fn new(element: E, deadline: F) -> Self {
        Self {
            element,
            ready: false,
            deadline,
        }
    }
}

impl<E, F> Element for DelayedUntil<E, F>
where
    E: Element,
    F: Future,
{
    type Effect = Option<E::Effect>;

    fn effect(&self) -> Self::Effect {
        if self.ready {
            // println!("[Delayed] Ready to show Element!");
            Some(self.element.effect())
        } else {
            // println!("[Delayed] Not ready.");
            None
        }
    }

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        let DelayedProj {
            element,
            ready,
            deadline: mut delay,
        } = self.project();

        loop {
            if *ready {
                return element.poll_change(cx);
            } else {
                let future_poll = delay.as_mut().poll(cx);

                match future_poll {
                    Poll::Ready(_) => {
                        *ready = true;
                    }
                    Poll::Pending => return Poll::Pending,
                }
            }
        }
    }
}

#[pin_project::pin_project(project = AwaitProj)]
pub struct Await<E, Maker, F> {
    maker: Maker,
    #[pin]
    element: OnceCell<E>,
    #[pin]
    deadline: F,
}

impl<E, Maker, F> Await<E, Maker, F> {
    pub fn new(deadline: F, maker: Maker) -> Self {
        Self {
            element: OnceCell::new(),
            maker,
            deadline,
        }
    }
}

impl<E, Maker, F> Element for Await<E, Maker, F>
where
    E: Element,
    F: Future,
    Maker: Fn(F::Output) -> E,
{
    type Effect = Option<E::Effect>;

    fn effect(&self) -> Self::Effect {
        self.element.get().map(|e| e.effect())
    }

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        let AwaitProj {
            element,
            maker,
            mut deadline,
        } = self.project();

        loop {
            match element.get() {
                Some(_) => {
                    let inner = unsafe { element.map_unchecked_mut(|e| e.get_mut().unwrap()) };
                    return inner.poll_change(cx);
                }
                None => {
                    let future_poll = deadline.as_mut().poll(cx);

                    match future_poll {
                        Poll::Ready(value) => {
                            let _ = OnceCell::set(&element, maker(value));
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                }
            }
        }
    }
}

pub trait FutureExt: Future {
    fn map_element<Maker, E>(self, maker: Maker) -> Await<E, Maker, Self>
    where
        Self: std::marker::Sized,
        Maker: Fn(Self::Output) -> E,
    {
        Await {
            maker,
            element: OnceCell::new(),
            deadline: self,
        }
    }
}
impl<F> FutureExt for F where F: Future {}

fn combine_polls(a: Poll<Option<()>>, b: Poll<Option<()>>) -> Poll<Option<()>> {
    use std::task::Poll::*;

    match (a, b) {
        (Ready(Some(())), _) | (_, Ready(Some(()))) => Ready(Some(())),
        (Pending, _) | (_, Pending) => Pending,
        _ => Ready(None),
    }
}
