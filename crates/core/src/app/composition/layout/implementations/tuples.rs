use std::pin::Pin;

use ::either::{Either, for_both, map_both};
use ui_composer_math::prelude::Size2;

use crate::app::composition::algebra::Combine;
use crate::app::composition::elements::{Blueprint, Element, Environment};
use crate::app::composition::layout::hints::{ChildHints, ParentHints};
use crate::app::composition::layout::Ui;

impl<Env: Environment> Ui<Env> for () {
    type Blueprint = ();

    fn prepare(&mut self, _: ParentHints) -> ChildHints {
        ChildHints {
            minimum_size: Size2::ZERO,
        }
    }

    fn place(&mut self, _: ParentHints, _: &Env::BlueprintResources<'_>) {}
    
    fn effect(&self) -> <<Self::Blueprint as crate::prelude::Blueprint<Env>>::Output as crate::prelude::Element<Env>>::Effect {
        
    }

    async fn propagate(&mut self, _: &mut ui_composer_input::event::Event) -> bool {
        false
    }
    
    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context,
        _: &<Env as crate::prelude::Environment>::BlueprintResources<'_>,
        _: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        std::task::Poll::Ready(None)
    }
}

impl<Env: Environment, A, B> Ui<Env> for (A, B)
where
    A: Ui<Env, Blueprint: Blueprint<Env>>,
    B: Ui<Env, Blueprint: Blueprint<Env>>,
{
    type Blueprint = (A::Blueprint, B::Blueprint);

    fn prepare(
        &mut self,
        parent_hints: ParentHints,
    ) -> crate::app::composition::layout::hints::ChildHints {
        let a = self.0.prepare(parent_hints);
        let b = self.0.prepare(parent_hints);
        ChildHints {
            minimum_size: Size2::max(a.minimum_size, b.minimum_size),
        }
    }

    fn place(&mut self, parent_hints: ParentHints, resources: &<Env as Environment>::BlueprintResources<'_>) {
        self.0.place(parent_hints, resources);
        self.1.place(parent_hints, resources);
    }
    
    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect {
        (
            self.0.effect(),
            self.1.effect()
        )
    }

    async fn propagate(&mut self, event: &mut ui_composer_input::event::Event) -> bool {
        Combine::combine(self.0.propagate(event).await, self.1.propagate(event).await)
    }
    
    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
        resources: &<Env as Environment>::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        let (pinned_a, pinned_b) = {
            let mut_ref = unsafe { self.get_unchecked_mut() };
            let (a, b) = mut_ref;

            let a = unsafe { Pin::new_unchecked(a) };
            let b = unsafe { Pin::new_unchecked(b) };

            (a, b)
        };

        let poll_a = pinned_a.poll_change(cx, resources, parent_hints);
        let poll_b = pinned_b.poll_change(cx, resources, parent_hints);

        Combine::combine(poll_a, poll_b)
    }
}

impl<Env, A> Ui<Env> for Box<A>
where
    A: Ui<Env, Blueprint: Blueprint<Env>> + ?Sized,
    Env: Environment
{
    type Blueprint = A::Blueprint;

    fn prepare(&mut self, parent_hints: ParentHints) -> ChildHints {
        self.as_mut().prepare(parent_hints)
    }

    fn place(&mut self, parent_hints: ParentHints, resources: &Env::BlueprintResources<'_>) {
        self.as_mut().place(parent_hints, resources);
    }
    
    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect {
        self.as_ref().effect()
    }

    fn propagate(&mut self, event: &mut ui_composer_input::event::Event) -> impl Future<Output = bool> {
        self.as_mut().propagate(event)
    }
    
    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context,
        resources: &<Env as Environment>::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        let inner = unsafe { self.map_unchecked_mut(|inner| &mut **inner) };
        inner.poll_change(cx, resources, parent_hints)
    }
}


impl<Env, A> Ui<Env> for Option<A>
where
    A: Ui<Env, Blueprint: Blueprint<Env>>,
    Env: Environment
{
    type Blueprint = Option<A::Blueprint>;

    fn prepare(&mut self, parent_hints: ParentHints) -> ChildHints {
        if let Some(inner) = self {
            inner.prepare(parent_hints)
        } else {
            ChildHints::default()
        }
    }

    fn place(&mut self, parent_hints: ParentHints, resources: &Env::BlueprintResources<'_>) {
        if let Some(inner) = self {
            inner.place(parent_hints, resources);
        }
    }
    
    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect {
        self.as_ref().map(|e| e.effect())
    }

    async fn propagate(&mut self, event: &mut ui_composer_input::event::Event) -> bool {
        if let Some(inner) = self {
            inner.propagate(event).await
        } else {
            false
        }
    }
    
    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context,
        resources: &<Env as Environment>::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        if let Some(inner) = self.as_pin_mut() {
            inner.poll_change(cx, resources, parent_hints)
        } else {
            ::std::task::Poll::Ready(None)
        }
    }
}

impl<Env, A, B> Ui<Env> for Either<A, B>
where
    A: Ui<Env, Blueprint: Blueprint<Env>>,
    B: Ui<Env, Blueprint: Blueprint<Env>>,
    Env: Environment
{
    type Blueprint = Either<A::Blueprint, B::Blueprint>;

    fn prepare(&mut self, parent_hints: ParentHints) -> ChildHints {
        for_both!(self, inner => inner.prepare(parent_hints))
    }

    fn place(&mut self, parent_hints: ParentHints, resources: &Env::BlueprintResources<'_>) {
        for_both!(self, inner => inner.place(parent_hints, resources))
    }
    
    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect {
        map_both!(self, inner => inner.effect())
    }

    async fn propagate(&mut self, event: &mut ui_composer_input::event::Event) -> bool {
        for_both!(self, inner => inner.propagate(event).await)
    }
    
    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context,
        resources: &<Env as Environment>::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        for_both!(self.as_pin_mut(), inner => inner.poll_change(cx, resources, parent_hints))
    }
}
