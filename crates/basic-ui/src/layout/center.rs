use ui_composer_core::app::composition::layout::{
    LayoutItem,
    hints::{ChildHints, ParentHints},
};
use vek::Rect;

/// A container that, as it is reshaped, keeps its item at its natural size and centered in the available space.
pub fn center<A>(item: A) -> CenterContainer<A>
where
    A: LayoutItem,
{
    CenterContainer {
        item,
        _item_hints_cache: ChildHints::default(),
    }
}

pub struct CenterContainer<A>
where
    A: LayoutItem,
{
    item: A,
    _item_hints_cache: ChildHints,
}

impl<A> LayoutItem for CenterContainer<A>
where
    A: LayoutItem,
{
    type Blueprint = A::Blueprint;

    fn prepare(&mut self, parent_hints: ParentHints) -> ChildHints {
        let hints = self.item.prepare(parent_hints);
        self._item_hints_cache = hints;
        hints
    }

    fn place(&mut self, parent_hints: ParentHints) -> Self::Blueprint {
        let my_rect = parent_hints.rect;
        let item_size = self._item_hints_cache.natural_size;
        let item_position =
            my_rect.position() + (my_rect.extent() - item_size) / 2.0;

        let item_rect = Rect::new(
            item_position.x,
            item_position.y,
            item_size.w,
            item_size.h,
        );

        let inner_hints = ParentHints {
            rect: item_rect,
            ..parent_hints
        };

        self.item.place(inner_hints)
    }
}
