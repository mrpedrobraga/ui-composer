use core::f32;

use ui_composer_core::app::composition::layout::{
    LayoutItem,
    hints::{ChildHints, ParentHints},
};
use ui_composer_math::prelude::{Rect, Size2, Vector2};

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
        __item_a_hints_cache: ChildHints::default(),
        __item_b_hints_cache: ChildHints::default(),
    }
}

pub struct RowContainer<A, B> {
    pub item_a: A,
    pub item_b: B,
    pub gap: f32,
    __item_a_hints_cache: ChildHints,
    __item_b_hints_cache: ChildHints,
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

    fn prepare(&mut self, parent_hints: ParentHints) -> ChildHints {
        let inner_hints = ParentHints {
            rect: Rect::new(
                parent_hints.rect.origin,
                Size2::new(f32::INFINITY, parent_hints.rect.size.height),
            ),
            ..parent_hints
        };
        let a = self.item_a.prepare(inner_hints);
        let b = self.item_b.prepare(inner_hints);

        self.__item_a_hints_cache = a;
        self.__item_b_hints_cache = b;

        let minimum_size = Size2::new(
            a.minimum_size.width + self.gap + b.minimum_size.width, // Min width with gap
            a.minimum_size.height.max(b.minimum_size.height), // Max height
        );
        let natural_size = Size2::new(
            a.natural_size.width + self.gap + b.natural_size.width, // Min width with gap
            a.natural_size.height.max(b.natural_size.height), // Max height
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
                    self.__item_a_hints_cache.natural_size.width,
                    parent_hints.rect.size.height,
                ),
            ),
            ..parent_hints
        });

        let b = self.item_b.place(ParentHints {
            rect: Rect::new(
                parent_hints.rect.origin.translate(Vector2::new(
                    self.__item_a_hints_cache.natural_size.width + self.gap,
                    0.0,
                )),
                Size2::new(
                    self.__item_b_hints_cache.natural_size.width,
                    parent_hints.rect.size.height,
                ),
            ),
            ..parent_hints
        });

        (a, b)
    }
}
