use ui_composer_core::app::composition::layout::{
    LayoutItem,
    hints::{ChildHints, ParentHints},
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
pub fn column<A, B>((item_a, item_b): (A, B)) -> ColumnContainer<A, B> {
    ColumnContainer {
        item_a,
        item_b,
        gap: 0.0,
        __item_a_hints_cache: ChildHints::default(),
        __item_b_hints_cache: ChildHints::default(),
    }
}

pub struct ColumnContainer<A, B> {
    pub item_a: A,
    pub item_b: B,
    pub gap: f32,
    __item_a_hints_cache: ChildHints,
    __item_b_hints_cache: ChildHints,
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

    fn prepare(
        &mut self,
        parent_hints: ParentHints,
    ) -> ui_composer_core::app::composition::layout::hints::ChildHints {
        let a = self.item_a.prepare(parent_hints);
        let b = self.item_b.prepare(parent_hints);

        self.__item_a_hints_cache = a;
        self.__item_b_hints_cache = b;

        let minimum_size = Extent2::new(
            a.minimum_size.w.max(b.minimum_size.w), // Max width
            a.minimum_size.h + self.gap + b.minimum_size.h, // Min height with gap
        );
        let natural_size = Extent2::new(
            a.natural_size.w.max(b.natural_size.w), // Max width
            a.natural_size.h + self.gap + b.natural_size.h, // Min height with gap
        );
        ChildHints {
            minimum_size,
            natural_size,
        }
    }

    fn place(&mut self, parent_hints: ParentHints) -> Self::Blueprint {
        let a = self.item_a.place(ParentHints {
            rect: Rect::new(
                parent_hints.rect.x,
                parent_hints.rect.y,
                parent_hints.rect.w,
                self.__item_a_hints_cache.natural_size.h,
            ),
            ..parent_hints
        });

        let b = self.item_b.place(ParentHints {
            rect: Rect::new(
                parent_hints.rect.x,
                parent_hints.rect.y
                    + self.__item_a_hints_cache.natural_size.h
                    + self.gap,
                parent_hints.rect.w,
                self.__item_b_hints_cache.natural_size.h,
            ),
            ..parent_hints
        });

        (a, b)
    }
}
