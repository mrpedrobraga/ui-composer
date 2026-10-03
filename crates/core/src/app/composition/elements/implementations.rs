use ::either::{map_both, Either};

use super::{Blueprint, Element};
use crate::app::composition::algebra::Combine;
use crate::app::composition::elements::Environment;
use crate::prelude::Empty;
use std::pin::Pin;
use std::task::{Context, Poll};

/* Unit */

impl<Env: Environment> Blueprint<Env> for () {
    type Output = ();

    fn make(self, _: &Env::BlueprintResources<'_>) -> Self::Output {}
}

impl<Env: Environment> Element<Env> for () {
    type Effect = ();
    type Blueprint = ();

    fn update(&mut self, _: Self::Blueprint, _: &<Env as Environment>::BlueprintResources<'_>) {}

    fn effect(&self) -> Self::Effect {}
}

/* Indirection */
impl<A, Env: Environment> Blueprint<Env> for Box<A>
where
    A: Blueprint<Env>,
{
    type Output = Box<A::Output>;

    fn make(self, env: &Env::BlueprintResources<'_>) -> Self::Output {
        Box::new(A::make(*self, env))
    }
}

impl<A, Env: Environment> Element<Env> for Box<A>
where
    A: Element<Env>,
{
    type Effect = A::Effect;
    type Blueprint = Box<A::Blueprint>;

    fn update(
        &mut self,
        blueprint: Self::Blueprint,
        resources: &<Env as Environment>::BlueprintResources<'_>,
    ) {
        self.as_mut().update(*blueprint, resources);
    }

    fn effect(&self) -> Self::Effect {
        let item = &**self;
        item.effect()
    }

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        env: &Env::BlueprintResources<'_>,
    ) -> Poll<Option<()>> {
        let mut_ref = unsafe { self.map_unchecked_mut(|e| &mut **e) };
        mut_ref.poll_change(cx, env)
    }
}

/* Tuples */

impl<A, B, Env: Environment> Blueprint<Env> for (A, B)
where
    A: Blueprint<Env>,
    B: Blueprint<Env>,
{
    type Output = (A::Output, B::Output);

    fn make(self, env: &Env::BlueprintResources<'_>) -> Self::Output {
        (self.0.make(env), self.1.make(env))
    }
}

impl<A, B, Env: Environment> Element<Env> for (A, B)
where
    A: Element<Env>,
    B: Element<Env>,
{
    type Effect = (A::Effect, B::Effect);
    type Blueprint = (A::Blueprint, B::Blueprint);

    fn update(&mut self, blueprint: Self::Blueprint, resources: &Env::BlueprintResources<'_>) {
        self.0.update(blueprint.0, resources);
        self.1.update(blueprint.1, resources);
    }

    fn effect(&self) -> Self::Effect {
        (self.0.effect(), self.1.effect())
    }

    fn poll_change(
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

        let poll_a = pinned_a.poll_change(cx, env);
        let poll_b = pinned_b.poll_change(cx, env);

        poll_a.combine(poll_b)
    }
}

impl<A, Env: Environment> Blueprint<Env> for Vec<A>
where
    A: Blueprint<Env>,
{
    type Output = Vec<A::Output>;

    fn make(self, env: &Env::BlueprintResources<'_>) -> Self::Output {
        self.into_iter().map(|it| it.make(env)).collect()
    }
}

impl<A, Env: Environment> Element<Env> for Vec<A>
where
    A: Element<Env>,
{
    type Effect = Vec<A::Effect>;
    type Blueprint = Vec<A::Blueprint>;

    fn update(
        &mut self,
        blueprint: Self::Blueprint,
        resources: &<Env as Environment>::BlueprintResources<'_>,
    ) {
        for (inner, blueprint) in self.iter_mut().zip(blueprint) {
            inner.update(blueprint, resources);
        }
    }

    fn effect(&self) -> Self::Effect {
        self.iter().map(|it| it.effect()).collect()
    }

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        env: &Env::BlueprintResources<'_>,
    ) -> Poll<Option<()>> {
        let items = unsafe { self.get_unchecked_mut() };
        items.iter_mut().fold(Empty::empty(), |acc, it| {
            let pinned = unsafe { Pin::new_unchecked(it) };
            acc.combine(pinned.poll_change(cx, env))
        })
    }
}

/* Options */

impl<A, Env: Environment> Blueprint<Env> for Option<A>
where
    A: Blueprint<Env>,
{
    type Output = Option<A::Output>;

    fn make(self, env: &Env::BlueprintResources<'_>) -> Self::Output {
        self.map(|x| x.make(env))
    }
}

impl<A, Env: Environment> Element<Env> for Option<A>
where
    A: Element<Env>,
{
    type Effect = Option<A::Effect>;
    type Blueprint = Option<A::Blueprint>;

    fn update(
        &mut self,
        blueprint: Self::Blueprint,
        resources: &<Env as Environment>::BlueprintResources<'_>,
    ) {
        if let Some(inner) = self {
            if let Some(bp) = blueprint {
                inner.update(bp, resources);
            } else {
                *self = None;
            }
        } else {
            if let Some(bp) = blueprint {
                *self = Some(bp.make(resources))
            }
        }
    }

    fn effect(&self) -> Self::Effect {
        self.as_ref().map(|inner| inner.effect())
    }

    fn poll_change(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &<Env as Environment>::BlueprintResources<'_>,
    ) -> Poll<Option<()>> {
        let projected_option = unsafe {
            self.as_mut()
                .get_unchecked_mut()
                .as_mut()
                .map(|value| Pin::new_unchecked(value))
        };

        match projected_option {
            Some(inner) => inner.poll_change(cx, resources),
            None => Poll::Ready(None),
        }
    }
}

/* Either */
impl<A, B, Env: Environment> Blueprint<Env> for Either<A, B>
where
    A: Blueprint<Env>,
    B: Blueprint<Env>,
{
    type Output = Either<A::Output, B::Output>;

    fn make(self, env: &Env::BlueprintResources<'_>) -> Self::Output {
        map_both!(self, x => x.make(env))
    }
}

impl<A, B, Env: Environment> Element<Env> for Either<A, B>
where
    A: Element<Env>,
    B: Element<Env>,
{
    type Effect = Either<A::Effect, B::Effect>;
    type Blueprint = Either<A::Blueprint, B::Blueprint>;

    fn update(
        &mut self,
        blueprint: Self::Blueprint,
        resources: &<Env as Environment>::BlueprintResources<'_>,
    ) {
        match self {
            Either::Left(current) => match blueprint {
                Either::Left(new_left) => {
                    current.update(new_left, resources);
                }
                Either::Right(_) => *self = blueprint.make(resources),
            },
            Either::Right(current) => match blueprint {
                Either::Left(_) => *self = blueprint.make(resources),
                Either::Right(new_right) => current.update(new_right, resources),
            },
        }
    }

    fn effect(&self) -> Self::Effect {
        map_both!(self, inner => inner.effect())
    }

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &<Env as Environment>::BlueprintResources<'_>,
    ) -> Poll<Option<()>> {
        match self.as_pin_mut() {
            Either::Left(inner) => inner.poll_change(cx, resources),
            Either::Right(inner) => inner.poll_change(cx, resources),
        }
    }
}
