use core::f32;

use ui_composer_core::app::composition::layout::{
    LayoutItem,
    hints::{ChildHints, ParentHints},
};
use ui_composer_math::prelude::{Rect, Size2, Vector2};

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
        let inner_hints = ParentHints {
            rect: Rect::new(
                parent_hints.rect.origin,
                Size2::new(parent_hints.rect.size.width, f32::INFINITY),
            ),
            ..parent_hints
        };
        let a = self.item_a.prepare(inner_hints);
        let b = self.item_b.prepare(inner_hints);

        self.__item_a_hints_cache = a;
        self.__item_b_hints_cache = b;

        let minimum_size = Size2::new(
            a.minimum_size.width.max(b.minimum_size.width), // Max width
            a.minimum_size.height + self.gap + b.minimum_size.height, // Min height with gap
        );
        let natural_size = Size2::new(
            a.natural_size.width.max(b.natural_size.width), // Max width
            a.natural_size.height + self.gap + b.natural_size.height, // Min height with gap
        );
        ChildHints {
            minimum_size,
            natural_size,
        }
    }

    fn place(&mut self, parent_hints: ParentHints) -> Self::Blueprint {
        let a = self.item_a.place(ParentHints {
            rect: Rect::new(
                parent_hints.rect.origin,
                Size2::new(
                    parent_hints.rect.size.width,
                    self.__item_a_hints_cache.natural_size.height,
                ),
            ),
            ..parent_hints
        });

        let b = self.item_b.place(ParentHints {
            rect: Rect::new(
                parent_hints.rect.origin.translate(Vector2::new(
                    0.0,
                    self.__item_a_hints_cache.natural_size.height + self.gap,
                )),
                Size2::new(
                    parent_hints.rect.size.width,
                    self.__item_b_hints_cache.natural_size.height,
                ),
            ),
            ..parent_hints
        });

        (a, b)
    }
}
