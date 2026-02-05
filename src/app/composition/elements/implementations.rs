use std::pin::Pin;
use std::task::{Context, Poll};
use crate::app::composition::algebra::Semigroup;
use super::{Blueprint, Element};

/* Unit */

impl <Env> Blueprint<Env> for () {
    type Element = ();

    fn make(self, env: &Env) -> Self::Element {
        ()
    }
}

impl<Env> Element<Env> for () {
    type Effect = ();

    fn effect(&self) -> Self::Effect {

    }
}

/* Tuples */

impl<A, B, Env> Blueprint<Env> for (A, B) where A: Blueprint<Env>, B: Blueprint<Env> {
    type Element = (A::Element, B::Element);

    fn make(self, env: &Env) -> Self::Element {
        (self.0.make(env), self.1.make(env))
    }
}

impl<A, B, Env> Element<Env> for (A, B)
where
    A: Element<Env>,
    B: Element<Env>,
{
    type Effect = (A::Effect, B::Effect);

    fn effect(&self) -> Self::Effect {
        (self.0.effect(), self.1.effect())
    }

    fn poll(self: Pin<&mut Self>, cx: &mut Context, env: &Env) -> Poll<Option<()>> {
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

/* Options */

impl<A, Env> Blueprint<Env> for Option<A> where A: Blueprint<Env> {
    type Element = Option<A::Element>;

    fn make(self, env: &Env) -> Self::Element {
        self.map(|x| x.make(env))
    }
}

impl<A, Env> Element<Env> for Option<A> where A: Element<Env> {
    type Effect = Option<A::Effect>;

    fn effect(&self) -> Self::Effect {
        self.as_ref().map(|x| x.effect())
    }
}