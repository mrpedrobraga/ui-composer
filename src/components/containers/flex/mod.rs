use crate::app::primitives::PrimitiveDescriptor;
use crate::prelude::flow::{CartesianFlowDirection, FlowDirection, WritingFlowDirection};
use crate::prelude::{CoordinateSystemProvider, LayoutItem, ParentHints};
use crate::winitwgpu::render_target::Render;
use std::iter::{once, Once};
use vek::{Extent2, Rect};

#[allow(non_snake_case)]
#[inline(always)]
/// A container that stretches some of its items to fill the remaining space.
pub fn Flex<TItems>(items: TItems) -> FlexContainer<TItems>
where
    TItems: FlexItems,
{
    FlexContainer {
        items,
        flow_direction: FlowDirection::Writing(WritingFlowDirection::WritingAxisForward),
    }
}

/// The struct created by [Flex].
pub struct FlexContainer<TItems: FlexItems> {
    items: TItems,
    flow_direction: FlowDirection,
}

impl<TItems: FlexItems> FlexContainer<TItems> {
    #[inline(always)]
    pub fn with_flow(self, flow_direction: FlowDirection) -> Self {
        Self {
            flow_direction,
            ..self
        }
    }

    #[inline(always)]
    /// Adapts this container to lay its items by [WritingFlowDirection::WritingCrossAxisForward]
    /// (in `en_US`, that's top to bottom).
    pub fn with_vertical_flow(self) -> Self {
        Self {
            flow_direction: FlowDirection::Writing(WritingFlowDirection::WritingCrossAxisForward),
            ..self
        }
    }
}

impl<TItems> LayoutItem for FlexContainer<TItems>
where
    TItems: FlexItems + Send,
    TItems::UINodeType: PrimitiveDescriptor + Render,
{
    type Content = TItems::UINodeType;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.items.get_natural_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.items.get_minimum_size()
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::Content {
        let flow_direction = self.flow_direction.as_cartesian(&parent_hints);
        let minima = self.items.minima(flow_direction).collect::<Vec<f32>>();
        let weights = self.items.weights().collect::<Vec<f32>>();

        use CartesianFlowDirection::*;
        let parent_size = match flow_direction {
            LeftToRight | RightToLeft => parent_hints.rect.w,
            TopToBottom | BottomToTop => parent_hints.rect.h,
        };

        let sizes = crate::prelude::flow::weighted_division_with_minima(
            parent_size,
            &weights,
            &minima,
            1.0,
        );
        let parent_hints =
            lay_sizes(parent_hints, flow_direction, sizes.into_iter()).collect::<Vec<_>>();

        self.items.lay(&parent_hints)
    }
}

fn lay_sizes<S>(
    container: ParentHints,
    flow_direction: CartesianFlowDirection,
    sizes: S,
) -> impl Iterator<Item = ParentHints>
where
    S: Iterator<Item = f32>,
{
    sizes.scan(0.0, move |offset_from_start, current_element_size| {
        use CartesianFlowDirection::*;

        let item_hints = match flow_direction {
            LeftToRight => ParentHints {
                rect: Rect::new(
                    container.rect.x + *offset_from_start,
                    container.rect.y,
                    current_element_size,
                    container.rect.h,
                ),
                ..container
            },
            RightToLeft => ParentHints {
                rect: Rect::new(
                    container.rect.x + container.rect.w - *offset_from_start - current_element_size,
                    container.rect.y,
                    current_element_size,
                    container.rect.h,
                ),
                ..container
            },
            TopToBottom => ParentHints {
                rect: Rect::new(
                    container.rect.x,
                    container.rect.y + *offset_from_start,
                    container.rect.w,
                    current_element_size,
                ),
                ..container
            },
            BottomToTop => ParentHints {
                rect: Rect::new(
                    container.rect.x,
                    container.rect.y + container.rect.w - *offset_from_start - current_element_size,
                    container.rect.w,
                    current_element_size,
                ),
                ..container
            },
        };

        *offset_from_start += current_element_size;

        Some(item_hints)
    })
}

/// An Item of a Flex Container, contains a LayoutItem and a weight.
pub struct FlexItem<T>(pub T, pub f32);

pub trait FlexItems {
    type UINodeType;
    type WeightsType: Iterator<Item = f32>;
    type MinimaType: Iterator<Item = f32>;

    fn get_natural_size(&self) -> Extent2<f32>;
    fn get_minimum_size(&self) -> Extent2<f32>;
    fn weights(&self) -> Self::WeightsType;
    fn minima(&self, flow_direction: CartesianFlowDirection) -> Self::WeightsType;
    fn lay(&mut self, hx: &[ParentHints]) -> Self::UINodeType;
}

impl<A> FlexItems for FlexItem<A>
where
    A: LayoutItem,
{
    type UINodeType = A::Content;
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

    fn minima(&self, flow_direction: CartesianFlowDirection) -> Once<f32> {
        use CartesianFlowDirection::*;

        match flow_direction {
            LeftToRight | RightToLeft => once(self.0.get_minimum_size().w),
            TopToBottom | BottomToTop => once(self.0.get_minimum_size().h),
        }
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

    fn minima(&self, flow_direction: CartesianFlowDirection) -> Self::WeightsType {
        Iterator::chain(self.0.minima(flow_direction), self.1.minima(flow_direction))
    }

    fn lay(&mut self, hx: &[ParentHints]) -> Self::UINodeType {
        let (a, b) = hx.split_at(1);
        (self.0.lay(a), self.1.lay(b))
    }
}
