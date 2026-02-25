use ui_composer_core::app::composition::layout::{
    LayoutItem, hints::ParentHints,
};
use vek::{Extent2, Rect};

/// A vertical, writing order stack of items.
///
/// ### Sizing
/// The height of the container is the sum of the heights
/// of the items inside (accounting for gap).
///
/// The width of the container is the max width between the items.
/// TODO: Allow to take more than two items.
pub fn column<A, B>(item_a: A, item_b: B) -> ColumnContainer<A, B> {
    ColumnContainer {
        item_a,
        item_b,
        gap: 0.0,
    }
}

pub struct ColumnContainer<A, B> {
    pub item_a: A,
    pub item_b: B,
    pub gap: f32,
}

impl<A, B> ColumnContainer<A, B> {
    /// Adds some spacing between elements.
    pub fn with_gap(self, gap: f32) -> Self {
        Self { gap, ..self }
    }
}

impl<A, B> LayoutItem for ColumnContainer<A, B>
where
    A: LayoutItem,
    B: LayoutItem,
{
    type Blueprint = (A::Blueprint, B::Blueprint);

    fn get_natural_size(&self) -> Extent2<f32> {
        let a_size = self.item_a.get_natural_size();

        let b_size = self.item_b.get_natural_size();

        // Adjust to stack vertically, considering the gap between the items
        Extent2::new(
            a_size.w.max(b_size.w), // Max width of the two items
            a_size.h + self.gap + b_size.h, // Sum of heights with the gap in between
        )
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        let a_size = self.item_a.get_minimum_size();

        let b_size = self.item_b.get_minimum_size();

        // Minimum size also stacks vertically
        Extent2::new(
            a_size.w.max(b_size.w),         // Max width
            a_size.h + self.gap + b_size.h, // Min height with gap
        )
    }

    fn place(&mut self, parent_hints: ParentHints) -> Self::Blueprint {
        let a_size = self.item_a.get_natural_size();

        let b_size = self.item_b.get_natural_size();

        let a = self.item_a.place(ParentHints {
            rect: Rect::new(
                parent_hints.rect.x,
                parent_hints.rect.y,
                parent_hints.rect.w,
                a_size.h,
            ),
            ..parent_hints
        });

        let b = self.item_b.place(ParentHints {
            rect: Rect::new(
                parent_hints.rect.x,
                parent_hints.rect.y + a_size.h + self.gap,
                parent_hints.rect.w,
                b_size.h,
            ),
            ..parent_hints
        });

        (a, b)
    }
}
