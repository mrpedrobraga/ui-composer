use ui_composer_math::prelude::Size2;

use crate::app::composition::layout::LayoutItem;
use crate::app::composition::layout::hints::{ChildHints, ParentHints};

impl LayoutItem for () {
    type Blueprint = ();

    fn prepare(&mut self, _: ParentHints) -> ChildHints {
        ChildHints {
            minimum_size: Size2::ZERO,
        }
    }

    fn place(&mut self, _: ParentHints) -> Self::Blueprint {}
}

impl<A, B> LayoutItem for (A, B)
where
    A: LayoutItem,
    B: LayoutItem,
{
    type Blueprint = (A::Blueprint, B::Blueprint);

    fn prepare(
        &mut self,
        parent_hints: ParentHints,
    ) -> crate::app::composition::layout::hints::ChildHints {
        let a = self.0.prepare(parent_hints);
        let b = self.0.prepare(parent_hints);
        ChildHints {
            minimum_size: a.minimum_size.max(b.minimum_size),
        }
    }

    fn place(&mut self, parent_hints: ParentHints) -> Self::Blueprint {
        (self.0.place(parent_hints), self.1.place(parent_hints))
    }
}

impl<A> LayoutItem for Box<A>
where
    A: LayoutItem + ?Sized,
{
    type Blueprint = A::Blueprint;

    fn prepare(&mut self, parent_hints: ParentHints) -> ChildHints {
        self.as_mut().prepare(parent_hints)
    }

    fn place(&mut self, parent_hints: ParentHints) -> Self::Blueprint {
        self.as_mut().place(parent_hints)
    }
}
