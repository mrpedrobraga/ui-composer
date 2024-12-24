#![allow(non_snake_case)]

use crate::{prelude::LayoutItem, ui::layout::ParentHints};
use vek::{Extent2, Rect};

/// A container that, as it is reshaped, keeps its item at its natural size, centered within itself.
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
    type UINodeType = A::UINodeType;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.item.get_natural_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.item.get_minimum_size()
    }

    fn lay(&mut self, layout_hints: ParentHints) -> Self::UINodeType {
        let my_rect = layout_hints.rect;

        let item_size = self.item.get_natural_size();
        let item_position = my_rect.position() + (my_rect.extent() - item_size) / 2.0;

        let item_rect = Rect::new(item_position.x, item_position.y, item_size.w, item_size.h);

        let inner_hints = ParentHints {
            rect: item_rect,
            ..layout_hints
        };

        self.item.lay(inner_hints)
    }
}

/// A container that scales its single item to a bigger size.
/// You **can not** make the minimum size _lower_ than the original, however.
pub fn WithSize<A>(suggested_size: Extent2<f32>, item: A) -> WithSizeContainer<A>
where
    A: LayoutItem,
{
    WithSizeContainer {
        suggested_size,
        item,
    }
}

pub struct WithSizeContainer<A>
where
    A: LayoutItem,
{
    suggested_size: Extent2<f32>,
    item: A,
}

impl<A> LayoutItem for WithSizeContainer<A>
where
    A: LayoutItem,
{
    type UINodeType = A::UINodeType;

    fn get_natural_size(&self) -> Extent2<f32> {
        let inner_size = self.item.get_natural_size();
        Extent2::new(
            f32::max(self.suggested_size.w, inner_size.w),
            f32::max(self.suggested_size.h, inner_size.h),
        )
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.item.get_minimum_size()
    }

    fn lay(&mut self, layout_hints: ParentHints) -> Self::UINodeType {
        self.item.lay(layout_hints)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ItemAlign {
    Stretchy,
    Sized(FlowPosition),
}

#[derive(Debug, Clone, Copy)]
pub enum FlowPosition {
    Start,
    Center,
    End,
}

/// A vertical, writing order stack of items.
///
/// ### Sizing
/// The height of the container is the sum of the heights
/// of the items inside (accounting for gap).
///
/// The width of the container is the max width between the items.
pub fn Column() {}

/// A horizontal, writing order stack of items.
///
/// ### Sizing
/// The width of the container is the sum of the widths
/// of the items inside (accounting for gap).
///
/// The height of the container is the max height between the items.
pub fn Row<A, B>(item_a: A, item_b: B) -> RowContainer<A, B> {
    RowContainer {
        item_a,
        item_b,
        gap: 8.0,
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
    type UINodeType = (A::UINodeType, B::UINodeType);

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

    fn lay(&mut self, parent_hints: ParentHints) -> Self::UINodeType {
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

#[derive(Debug, Clone, Copy)]
pub enum FlexItem<T> {
    /// This item has a defined size (its own natural, usually minimum, size).
    Sized(T),
    /// This item has a minimum size but will grow to fit empty space (with a weight).
    Stretchy(f32, T),
}

/// An main-axis heap of ordered items, where some of the items might stretch with an assigned weight.
///
/// ### Overflow
/// [`Flex`] asks you for a "suggested_size" that it'll use for its minimum size...
/// This size is only a suggestion, and items can force the minimum size to be bigger than it
/// both on the main and on the cross axis.
pub fn Flex() {}

/// A cross-axis stack of [`Flex`]s, where their items are organized so that they fit.
pub fn FlexWrap() {}
