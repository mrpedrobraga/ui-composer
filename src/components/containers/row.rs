use {
    crate::prelude::{LayoutItem, ParentHints},
    vek::{Extent2, Rect},
};

/// A horizontal, writing order stack of items.
///
/// ### Sizing
/// The width of the container is the sum of the widths
/// of the items inside (accounting for gap).
///
/// The height of the container is the max height between the items.
/// TODO: Allow it to take more than two items.
pub fn Row<A, B>(item_a: A, item_b: B) -> RowContainer<A, B> {
    RowContainer {
        item_a,
        item_b,
        gap: 0.0,
    }
}

pub struct RowContainer<A, B> {
    item_a: A,
    item_b: B,
    gap: f32,
}

impl<A, B> LayoutItem for RowContainer<A, B>
where
    A: LayoutItem,
    B: LayoutItem,
{
    type UIItemType = (A::UIItemType, B::UIItemType);

    fn get_natural_size(&self) -> Extent2<f32> {
        let a_size = self.item_a.get_natural_size();
        let b_size = self.item_b.get_natural_size();

        Extent2::new(a_size.w + self.gap + b_size.w, a_size.h.max(b_size.h))
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        let a_size = self.item_a.get_minimum_size();
        let b_size = self.item_b.get_minimum_size();

        Extent2::new(a_size.w + self.gap + b_size.w, a_size.h.max(b_size.h))
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::UIItemType {
        let a_size = self.item_a.get_natural_size();
        let b_size = self.item_b.get_natural_size();

        let a = self.item_a.lay(ParentHints {
            rect: Rect::new(
                parent_hints.rect.x,
                parent_hints.rect.y,
                a_size.w,
                parent_hints.rect.h,
            ),
            ..parent_hints
        });

        let b = self.item_b.lay(ParentHints {
            rect: Rect::new(
                parent_hints.rect.x + a_size.w + self.gap,
                parent_hints.rect.y,
                b_size.w,
                parent_hints.rect.h,
            ),
            ..parent_hints
        });

        (a, b)
    }
}
