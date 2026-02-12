use crate::app::composition::layout::LayoutItem;
use crate::app::composition::layout::hints::ParentHints;
use vek::{Extent2, Rect};

/// A container that, as it is reshaped, keeps its item at its natural size and centered in the available space.
pub fn Center<A>(item: A) -> CenterContainer<A>
where
    A: LayoutItem,
{
    CenterContainer { item }
}

pub struct CenterContainer<A>
where
    A: LayoutItem,
{
    item: A,
}

impl<A> LayoutItem for CenterContainer<A>
where
    A: LayoutItem,
{
    type Blueprint = A::Blueprint;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.item.get_natural_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.item.get_minimum_size()
    }

    fn lay(&mut self, layout_hints: ParentHints) -> Self::Blueprint {
        let my_rect = layout_hints.rect;
        let item_size = self.item.get_natural_size();
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
            ..layout_hints
        };

        self.item.lay(inner_hints)
    }
}
