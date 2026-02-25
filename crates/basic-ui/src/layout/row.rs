use ui_composer_core::app::composition::layout::{
    LayoutItem, hints::ParentHints,
};
use vek::{Extent2, Rect};

/// A horizontal, writing order stack of items.
///
/// ### Sizing
/// The width of the container is the sum of the widths
/// of the items inside (accounting for gap).
///
/// The height of the container is the max height between the items.
/// TODO: Allow it to take more than two items.
pub fn row<A, B>((item_a, item_b): (A, B)) -> RowContainer<A, B> {
    RowContainer {
        item_a,
        item_b,
        gap: 0.0,
    }
}

pub struct RowContainer<A, B> {
    pub item_a: A,
    pub item_b: B,
    pub gap: f32,
}

impl<A, B> RowContainer<A, B> {
    pub fn with_gap(self, gap: f32) -> Self {
        Self { gap, ..self }
    }
}

impl<A, B> LayoutItem for RowContainer<A, B>
where
    A: LayoutItem,
    B: LayoutItem,
{
    type Blueprint = (A::Blueprint, B::Blueprint);

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

    fn place(&mut self, parent_hints: ParentHints) -> Self::Blueprint {
        let a_size = self.item_a.get_natural_size();
        let b_size = self.item_b.get_natural_size();

        let a = self.item_a.place(ParentHints {
            rect: Rect::new(
                parent_hints.rect.x,
                parent_hints.rect.y,
                a_size.w,
                parent_hints.rect.h,
            ),
            ..parent_hints
        });

        let b = self.item_b.place(ParentHints {
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
