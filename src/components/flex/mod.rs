use crate::prelude::functions::weighted_division_with_minima;
use crate::prelude::{ItemDescriptor, LayoutItem, ParentHints};
use std::iter::{once, Once};
use vek::{Extent2, Rect};

/// An Item of a Flex Container, contains a LayoutItem and a weight.
pub struct FlexItem<T>(pub T, pub f32);

/// A container that stretches some of its items to fill the remaining space.
pub struct FlexContainer<TItems: FlexItems> {
    items: TItems,
}

#[allow(non_snake_case)]
/// A container that stretches some of its items to fill the remaining space.
pub fn Flex<TItems>(items: TItems) -> FlexContainer<TItems>
where
    TItems: FlexItems,
{
    FlexContainer { items }
}

pub trait FlexItems {
    type UINodeType: ItemDescriptor;
    type WeightsType: Iterator<Item = f32>;
    type MinimaType: Iterator<Item = f32>;

    fn get_natural_size(&self) -> Extent2<f32>;
    fn get_minimum_size(&self) -> Extent2<f32>;
    fn weights(&self) -> Self::WeightsType;
    fn minima(&self) -> Self::WeightsType;
    fn lay(&mut self, hx: &[ParentHints]) -> Self::UINodeType;
}

impl<A> FlexItems for FlexItem<A>
where
    A: LayoutItem,
{
    type UINodeType = A::UINodeType;
    type WeightsType = Once<f32>;
    type MinimaType = Once<f32>;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.0.get_natural_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.0.get_minimum_size()
    }

    fn weights(&self) -> Once<f32> {
        once(self.1)
    }

    fn minima(&self) -> Once<f32> {
        once(self.0.get_minimum_size().w)
    }

    fn lay(&mut self, hx: &[ParentHints]) -> Self::UINodeType {
        self.0.lay(hx[0])
    }
}

impl<A, B> FlexItems for (A, B)
where
    A: FlexItems,
    B: FlexItems,
{
    type UINodeType = (A::UINodeType, B::UINodeType);

    type WeightsType = std::iter::Chain<A::WeightsType, B::WeightsType>;

    type MinimaType = std::iter::Chain<A::MinimaType, B::MinimaType>;

    fn get_natural_size(&self) -> Extent2<f32> {
        let a_size = self.0.get_natural_size();
        let b_size = self.1.get_natural_size();

        Extent2::new(a_size.w + b_size.w, a_size.h.max(b_size.h))
    }
    fn get_minimum_size(&self) -> Extent2<f32> {
        let a_size = self.0.get_minimum_size();
        let b_size = self.1.get_minimum_size();

        Extent2::new(a_size.w + b_size.w, a_size.h.max(b_size.h))
    }

    fn weights(&self) -> Self::WeightsType {
        self.0.weights().chain(self.1.weights())
    }

    fn minima(&self) -> Self::WeightsType {
        self.0.minima().chain(self.1.minima())
    }

    fn lay(&mut self, hx: &[ParentHints]) -> Self::UINodeType {
        let (a, b) = hx.split_at(1);
        (self.0.lay(a), self.1.lay(b))
    }
}

impl<TItems> LayoutItem for FlexContainer<TItems>
where
    TItems: FlexItems + Send,
{
    type UINodeType = TItems::UINodeType;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.items.get_natural_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.items.get_minimum_size()
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::UINodeType {
        let minima = self.items.minima().collect::<Vec<f32>>();
        let weights = self.items.weights().collect::<Vec<f32>>();

        let sizes = weighted_division_with_minima(parent_hints.rect.w, &weights, &minima, 1.0);
        let parent_hints = sizes
            .into_iter()
            .scan(0.0, |acc, size| {
                let item_hints = ParentHints {
                    rect: Rect::new(
                        parent_hints.rect.x + *acc,
                        parent_hints.rect.y,
                        size,
                        parent_hints.rect.h,
                    ),
                    ..parent_hints
                };
                *acc += size;

                Some(item_hints)
            })
            .collect::<Vec<_>>();

        self.items.lay(&parent_hints)
    }
}
