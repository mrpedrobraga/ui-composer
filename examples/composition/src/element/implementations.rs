use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::runner::{ElementEffectHandler, InitializationResources};

use super::{max, Blueprint, Element, ElementEffect, Ui};

impl<A, B> Blueprint for (A, B)
where
    A: Blueprint,
    B: Blueprint,
{
    type Output = (A::Output, B::Output);

    fn make(self, resources: &InitializationResources) -> Self::Output {
        // TODO: Split the resources instead of sharing,
        // so they can be used in parallel and also
        // different parts of the app can grab dibs on different
        // parts of the resources.
        (self.0.make(resources), self.1.make(resources))
    }
}

impl<A, B> Element for (A, B)
where
    A: Element,
    B: Element,
{
    type Effect = (A::Effect, B::Effect);
    type Blueprint = (A::Blueprint, B::Blueprint);

    fn update(&mut self, blueprint: Self::Blueprint, resources: &InitializationResources) {
        self.0.update(blueprint.0, resources);
        self.1.update(blueprint.1, resources);
    }

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

        max(poll_a, poll_b)
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

impl<A> Blueprint for Option<A>
where
    A: Blueprint,
{
    type Output = Option<A::Output>;

    fn make(self, resources: &InitializationResources) -> Self::Output {
        self.map(|inner| inner.make(resources))
    }
}

impl<A> Element for Option<A>
where
    A: Element,
{
    type Effect = Option<A::Effect>;

    type Blueprint = Option<A::Blueprint>;

    fn update(&mut self, blueprint: Self::Blueprint, resources: &InitializationResources) {
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

    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        let projected_option = unsafe {
            self.as_mut()
                .get_unchecked_mut()
                .as_mut()
                .map(|value| Pin::new_unchecked(value))
        };

        match projected_option {
            Some(inner) => inner.poll_change(cx),
            None => Poll::Ready(None),
        }
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

impl<A> Ui for Option<A>
where
    A: Ui,
{
    type Blueprint = Option<A::Blueprint>;

    fn place(&mut self, parent_hints: super::ParentHints, resources: &InitializationResources) {
        if let Some(inner) = self.as_mut() {
            inner.place(parent_hints, resources)
        }
    }

    fn effect(&self) -> <<Self::Blueprint as Blueprint>::Output as Element>::Effect {
        self.as_ref().map(|inner| inner.effect())
    }

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &InitializationResources,
        parent_hints: super::ParentHints,
    ) -> Poll<Option<()>> {
        if let Some(inner) = self.as_pin_mut() {
            inner.poll_change(cx, resources, parent_hints)
        } else {
            Poll::Ready(None)
        }
    }
}
