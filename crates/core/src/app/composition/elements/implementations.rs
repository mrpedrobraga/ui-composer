use super::{Blueprint, Element};
use crate::app::composition::algebra::Semigroup;
use crate::app::composition::elements::Environment;
use crate::prelude::Empty;
use std::pin::Pin;
use std::task::{Context, Poll};

/* Unit */

impl<Env: Environment> Blueprint<Env> for () {
    type Element = ();

    fn make(self, _: &Env::BlueprintResources<'_>) -> Self::Element {}
}

impl<Env: Environment> Element<Env> for () {
    type Effect = ();

    fn effect(&self) -> Self::Effect {}
}

/* Indirection */
impl<A, Env: Environment> Blueprint<Env> for Box<A>
where
    A: Blueprint<Env>,
{
    type Element = Box<A::Element>;

    fn make(self, env: &Env::BlueprintResources<'_>) -> Self::Element {
        Box::new(A::make(*self, env))
    }
}

impl<A, Env: Environment> Element<Env> for Box<A>
where
    A: Element<Env>,
{
    type Effect = A::Effect;

    fn effect(&self) -> Self::Effect {
        let item = &**self;
        item.effect()
    }

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context,
        env: &Env::BlueprintResources<'_>,
    ) -> Poll<Option<()>> {
        let mut_ref = unsafe { self.map_unchecked_mut(|e| &mut **e) };
        mut_ref.poll(cx, env)
    }
}

/* Tuples */

impl<A, B, Env: Environment> Blueprint<Env> for (A, B)
where
    A: Blueprint<Env>,
    B: Blueprint<Env>,
{
    type Element = (A::Element, B::Element);

    fn make(self, env: &Env::BlueprintResources<'_>) -> Self::Element {
        (self.0.make(env), self.1.make(env))
    }
}

impl<A, B, Env: Environment> Element<Env> for (A, B)
where
    A: Element<Env>,
    B: Element<Env>,
{
    type Effect = (A::Effect, B::Effect);

    fn effect(&self) -> Self::Effect {
        (self.0.effect(), self.1.effect())
    }

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context,
        env: &Env::BlueprintResources<'_>,
    ) -> Poll<Option<()>> {
        let (pinned_a, pinned_b) = {
            let mut_ref = unsafe { self.get_unchecked_mut() };
            let (a, b) = mut_ref;

            let a = unsafe { Pin::new_unchecked(a) };
            let b = unsafe { Pin::new_unchecked(b) };

            (a, b)
        };

        let poll_a = pinned_a.poll(cx, env);
        let poll_b = pinned_b.poll(cx, env);

        poll_a.combine(poll_b)
    }
}

impl<A, Env: Environment> Blueprint<Env> for Vec<A>
where
    A: Blueprint<Env>,
{
    type Element = Vec<A::Element>;

    fn make(self, env: &Env::BlueprintResources<'_>) -> Self::Element {
        self.into_iter().map(|it| it.make(env)).collect()
    }
}

impl<A, Env: Environment> Element<Env> for Vec<A>
where
    A: Element<Env>,
{
    type Effect = Vec<A::Effect>;

    fn effect(&self) -> Self::Effect {
        self.iter().map(|it| it.effect()).collect()
    }

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context,
        env: &Env::BlueprintResources<'_>,
    ) -> Poll<Option<()>> {
        let items = unsafe { self.get_unchecked_mut() };
        items.iter_mut().fold(Empty::empty(), |acc, it| {
            let pinned = unsafe { Pin::new_unchecked(it) };
            acc.combine(pinned.poll(cx, env))
        })
    }
}

/* Options */

impl<A, Env: Environment> Blueprint<Env> for Option<A>
where
    A: Blueprint<Env>,
{
    type Element = Option<A::Element>;

    fn make(self, env: &Env::BlueprintResources<'_>) -> Self::Element {
        self.map(|x| x.make(env))
    }
}

impl<A, Env: Environment> Element<Env> for Option<A>
where
    A: Element<Env>,
{
    type Effect = Option<A::Effect>;

    fn effect(&self) -> Self::Effect {
        self.as_ref().map(|x| x.effect())
    }

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        env: &<Env as Environment>::BlueprintResources<'_>,
    ) -> Poll<Option<()>> {
        let projected_option = unsafe {
            self.as_mut()
                .get_unchecked_mut()
                .as_mut()
                .map(|value| Pin::new_unchecked(value))
        };

        match projected_option {
            Some(inner) => inner.poll(cx, env),
            // TODO: Ideally, something like `Option<()>` could return `Ready(None)`?
            None => Poll::Pending,
        }
    }
}
