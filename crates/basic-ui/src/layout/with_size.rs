use ui_composer_core::app::composition::layout::{
    LayoutItem,
    hints::{ChildHints, ParentHints},
};
use ui_composer_math::prelude::Size2;

pub struct WithSizeContainer<A>
where
    A: LayoutItem,
{
    suggested_size: Size2,
    item: A,
}

/// A container that scales its single item to a bigger size.
/// You **can not** make the minimum size _lower_ than the original, however.
pub fn with_size<A>(item: A) -> WithSizeContainer<A>
where
    A: LayoutItem,
{
    WithSizeContainer {
        suggested_size: Size2::ZERO,
        item,
    }
}

impl<A> WithSizeContainer<A>
where
    A: LayoutItem,
{
    pub fn with_size(self, suggested_size: Size2) -> Self {
        Self {
            suggested_size,
            ..self
        }
    }
}

impl<A> LayoutItem for WithSizeContainer<A>
where
    A: LayoutItem,
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

    fn place(&mut self, layout_hints: ParentHints) -> Self::Blueprint {
        self.item.place(layout_hints)
    }
}
