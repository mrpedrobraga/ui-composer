use std::marker::PhantomData;

use ui_composer_core::app::composition::{
    elements::Environment,
    layout::{
        hints::{ChildHints, ParentHints},
        Ui,
    },
};
use ui_composer_math::prelude::Size2;

#[pin_project::pin_project]
pub struct WithSizeContainer<Env, A>
where
    A: Ui<Env>,
    Env: Environment,
{
    suggested_size: Size2,
    #[pin]
    item: A,
    __marker: PhantomData<Env>,
}

/// A container that scales its single item to a bigger size.
/// You **can not** make the minimum size _lower_ than the original, however.
pub fn with_size<Env, A>(item: A) -> WithSizeContainer<Env, A>
where
    A: Ui<Env>,
    Env: Environment,
{
    WithSizeContainer {
        suggested_size: Size2::ZERO,
        item,
        __marker: PhantomData,
    }
}

impl<Env, A> WithSizeContainer<Env, A>
where
    A: Ui<Env>,
    Env: Environment,
{
    pub fn with_size(self, suggested_size: Size2) -> Self {
        Self {
            suggested_size,
            ..self
        }
    }
}

impl<Env, A> Ui<Env> for WithSizeContainer<Env, A>
where
    A: Ui<Env>,
    Env: Environment,
{
    type Blueprint = A::Blueprint;

    fn prepare(
        &mut self,
        parent_hints: ParentHints,
    ) -> ui_composer_core::app::composition::layout::hints::ChildHints {
        let inner = self.item.prepare(parent_hints);
        ChildHints {
            minimum_size: self.suggested_size.max(inner.minimum_size),
        }
    }

    fn place(&mut self, layout_hints: ParentHints, resources: &Env::BlueprintResources<'_>) {
        self.item.place(layout_hints, resources);
    }
    
    fn effect(&self) -> <<Self::Blueprint as ui_composer_core::prelude::Blueprint<Env>>::Output as ui_composer_core::prelude::Element<Env>>::Effect {
        self.item.effect()
    }
    
    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
        resources: &<Env as Environment>::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        let this = self.project();
        this.item.poll_change(cx, resources, parent_hints)
    }
}
