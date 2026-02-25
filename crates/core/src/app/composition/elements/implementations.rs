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
    type Effect<'fx> = ();

    fn effect(&self) -> Self::Effect<'_> {}
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
    type Effect<'fx>
        = A::Effect<'fx>
    where
        A: 'fx;

    fn effect(&self) -> Self::Effect<'_> {
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
    type Effect<'fx>
        = (A::Effect<'fx>, B::Effect<'fx>)
    where
        A: 'fx,
        B: 'fx;

    fn effect(&self) -> Self::Effect<'_> {
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
    type Effect<'fx>
        = Vec<A::Effect<'fx>>
    where
        A: 'fx;

    fn effect(&self) -> Self::Effect<'_> {
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
    type Effect<'fx>
        = Option<A::Effect<'fx>>
    where
        A: 'fx;

    fn effect(&self) -> Self::Effect<'_> {
        self.as_ref().map(|x| x.effect())
    }
}
