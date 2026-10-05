use std::marker::PhantomData;

use ui_composer_core::app::composition::{
    elements::{Blueprint, Element, Environment},
    layout::{
        Ui,
        hints::{ChildHints, ParentHints},
    },
};
use ui_composer_math::prelude::Rect;

/// A container that, as it is reshaped, keeps its item at its natural size and centered in the available space.
pub fn center<Env, A>(item: A) -> CenterContainer<Env, A>
where
    A: Ui<Env>,
    Env: Environment,
{
    CenterContainer {
        item,
        _marker: PhantomData,
        _item_hints_cache: ChildHints::default(),
    }
}

#[pin_project::pin_project]
pub struct CenterContainer<Env, A>
where
    A: Ui<Env>,
    Env: Environment,
{
    #[pin]
    item: A,
    _marker: PhantomData<Env>,
    _item_hints_cache: ChildHints,
}


impl<Env, A> Ui<Env> for CenterContainer<Env, A>
where
    A: Ui<Env>,
    Env: Environment,
{
    type Blueprint = A::Blueprint;

    fn prepare(&mut self, parent_hints: ParentHints) -> ChildHints {
        let hints = self.item.prepare(parent_hints);
        self._item_hints_cache = hints;
        hints
    }

    fn place(
        &mut self,
        parent_hints: ParentHints,
        resources: &Env::BlueprintResources<'_>,
    ) {
        let my_rect = parent_hints.rect;
        let item_size = self._item_hints_cache.minimum_size;
        let item_position =
            my_rect.origin + (my_rect.size - item_size).to_vector() / 2.0;

        let item_rect = Rect::new(item_position, item_size);

        let inner_hints = ParentHints {
            rect: item_rect,
            ..parent_hints
        };

        self.item.place(inner_hints, resources);
    }

    fn effect(
        &self,
    ) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect
    {
        self.item.effect()
    }

    async fn propagate(
        &mut self,
        event: &mut ui_composer_input::event::Event,
    ) -> bool {
        self.item.propagate(event).await
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
        resources: &<Env as Environment>::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        let my_rect = parent_hints.rect;
        let item_size = self._item_hints_cache.minimum_size;
        let item_position =
            my_rect.origin + (my_rect.size - item_size).to_vector() / 2.0;

        let item_rect = Rect::new(item_position, item_size);

        let inner_hints = ParentHints {
            rect: item_rect,
            ..parent_hints
        };

        let this = self.project();
        this.item.poll_change(cx, resources, inner_hints)
    }
}
