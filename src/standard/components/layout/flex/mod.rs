use crate::app::composition::layout::hints::ParentHints;
use crate::app::composition::layout::{CoordinateSystem, LayoutItem};
use crate::geometry::flow::CartesianFlow::{
    BottomToTop, LeftToRight, RightToLeft, TopToBottom,
};
use crate::geometry::flow::{CartesianFlow, Flow, WritingFlow};
use crate::prelude::flow::arrangers::arrange_stretchy_rects_with_minimum_sizes_dirty_alloc;
use core::iter::{Chain, Once, once};
use vek::{Extent2, Rect};

#[allow(non_snake_case)]
#[inline(always)]
/// A container that stretches some of its items to fill the remaining space.
pub fn flex<TItems>(items: TItems) -> FlexContainer<TItems>
where
    TItems: FlexItemList,
{
    FlexContainer {
        items,
        flow_direction: Flow::Writing(WritingFlow::WritingAxisForward),
    }
}

/// The struct created by [flex].
pub struct FlexContainer<TItems: FlexItemList> {
    items: TItems,
    flow_direction: Flow,
}

impl<TItems: FlexItemList> FlexContainer<TItems> {
    #[inline(always)]
    pub fn with_flow(self, flow_direction: Flow) -> Self {
        Self {
            flow_direction,
            ..self
        }
    }

    #[inline(always)]
    /// Adapts this container to lay its items by [WritingFlow::WritingCrossAxisForward]
    /// (in `en_US`, that's top to bottom).
    pub fn with_vertical_flow(self) -> Self {
        Self {
            flow_direction: Flow::Writing(WritingFlow::WritingCrossAxisForward),
            ..self
        }
    }
}

impl<ItemList> LayoutItem for FlexContainer<ItemList>
where
    ItemList: FlexItemList + Send,
{
    type Blueprint = ItemList::Content;

    fn get_natural_size(&self) -> Extent2<f32> {
        let item_natural_sizes = self.items.get_natural_sizes();

        // TODO: Receive the parent hints from... well, the parent.
        let flow_direction = self.flow_direction.as_cartesian(&ParentHints {
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            current_flow_direction: CartesianFlow::LeftToRight,
            current_cross_flow_direction: CartesianFlow::TopToBottom,
            current_writing_flow_direction: CartesianFlow::LeftToRight,
            current_writing_cross_flow_direction: CartesianFlow::TopToBottom,
        });

        match flow_direction {
            LeftToRight | RightToLeft => item_natural_sizes
                .reduce(|a, b| Extent2::new(a.w + b.w, a.h.max(b.h))),
            TopToBottom | BottomToTop => item_natural_sizes
                .reduce(|a, b| Extent2::new(a.w.max(b.w), a.h + b.h)),
        }
        .unwrap_or_default()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        let item_min_sizes = self.items.get_minimum_sizes();

        // TODO: Receive the parent hints from... well, the parent.
        let flow_direction = self.flow_direction.as_cartesian(&ParentHints {
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            current_flow_direction: LeftToRight,
            current_cross_flow_direction: TopToBottom,
            current_writing_flow_direction: LeftToRight,
            current_writing_cross_flow_direction: TopToBottom,
        });

        match flow_direction {
            LeftToRight | RightToLeft => item_min_sizes
                .reduce(|a, b| Extent2::new(a.w + b.w, a.h.max(b.h))),
            TopToBottom | BottomToTop => item_min_sizes
                .reduce(|a, b| Extent2::new(a.w.max(b.w), a.h + b.h)),
        }
        .unwrap_or_default()
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::Blueprint {
        let flow_direction = self.flow_direction.as_cartesian(&parent_hints);
        let minima = self
            .items
            .minima(flow_direction)
            //.collect::<ArrayVec<_, SIZE>>();
            .collect::<Vec<_>>();
        let weights = self
            .items
            .weights()
            //.collect::<ArrayVec<_, SIZE>>();
            .collect::<Vec<_>>();

        use CartesianFlow::*;
        let parent_size = match flow_direction {
            LeftToRight | RightToLeft => parent_hints.rect.w,
            TopToBottom | BottomToTop => parent_hints.rect.h,
        };

        /*let main_axis_sizes = weighted_division_with_minima(
            parent_size,
            &weights.into_inner().unwrap(),
            &minima.into_inner().unwrap(),
            0.01,
        );*/
        let main_axis_sizes = arrange_stretchy_rects_with_minimum_sizes_dirty_alloc(
            parent_size,
            weights.as_slice(),
            minima.as_slice(),
            0.01,
        );
        let parent_hints = lay_sizes(
            parent_hints,
            flow_direction,
            main_axis_sizes.into_iter(),
        );

        self.items.lay(parent_hints)
    }
}

fn lay_sizes<S>(
    container: ParentHints,
    flow_direction: CartesianFlow,
    sizes: S,
) -> impl Iterator<Item = ParentHints>
where
    S: Iterator<Item = f32>,
{
    sizes.scan(0.0, move |offset_from_start, current_element_size| {
        use CartesianFlow::*;

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
                    container.rect.x + container.rect.w
                        - *offset_from_start
                        - current_element_size,
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
                    container.rect.y + container.rect.w
                        - *offset_from_start
                        - current_element_size,
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
pub struct FlexItem<T> {
    item: T,
    grow: f32,
}
pub fn item<T>(item: T) -> FlexItem<T> {
    FlexItem { item, grow: 0.0 }
}
impl<T> FlexItem<T> {
    pub fn with_grow(self, grow: f32) -> FlexItem<T> {
        Self { grow, ..self }
    }
}

pub trait FlexItemList {
    type Content;
    type Weights: Iterator<Item = f32>;
    type Minima: Iterator<Item = f32>;
    const SIZE: usize;

    fn get_natural_sizes(&self) -> impl Iterator<Item = Extent2<f32>>;
    fn get_minimum_sizes(&self) -> impl Iterator<Item = Extent2<f32>>;
    fn weights(&self) -> Self::Weights;
    fn minima(&self, flow_direction: CartesianFlow) -> Self::Weights;
    fn lay<I>(&mut self, hx: I) -> Self::Content
    where
        I: Iterator<Item = ParentHints>;
}

impl<A> FlexItemList for FlexItem<A>
where
    A: LayoutItem,
{
    type Content = A::Blueprint;
    type Weights = Once<f32>;
    type Minima = Once<f32>;
    const SIZE: usize = 1;

    fn get_natural_sizes(&self) -> impl Iterator<Item = Extent2<f32>> {
        once(self.item.get_natural_size())
    }

    fn get_minimum_sizes(&self) -> impl Iterator<Item = Extent2<f32>> {
        once(self.item.get_minimum_size())
    }

    fn weights(&self) -> Once<f32> {
        once(self.grow)
    }

    fn minima(&self, flow_direction: CartesianFlow) -> Once<f32> {
        use CartesianFlow::*;

        match flow_direction {
            LeftToRight | RightToLeft => once(self.item.get_minimum_size().w),
            TopToBottom | BottomToTop => once(self.item.get_minimum_size().h),
        }
    }

    fn lay<I>(&mut self, mut hx: I) -> Self::Content
    where
        I: Iterator<Item = ParentHints>,
    {
        self.item.lay(hx.next().unwrap()) //NOTE: Make sure to send an element or else...
    }
}

impl<A, B> FlexItemList for (A, B)
where
    A: FlexItemList,
    B: FlexItemList,
{
    type Content = (A::Content, B::Content);
    type Weights = Chain<A::Weights, B::Weights>;
    type Minima = Chain<A::Minima, B::Minima>;
    const SIZE: usize = A::SIZE + B::SIZE;

    fn get_natural_sizes(&self) -> impl Iterator<Item = Extent2<f32>> {
        self.0.get_natural_sizes().chain(self.1.get_natural_sizes())
    }
    fn get_minimum_sizes(&self) -> impl Iterator<Item = Extent2<f32>> {
        self.0.get_minimum_sizes().chain(self.1.get_minimum_sizes())
    }

    fn weights(&self) -> Self::Weights {
        self.0.weights().chain(self.1.weights())
    }

    fn minima(&self, flow_direction: CartesianFlow) -> Self::Weights {
        Iterator::chain(
            self.0.minima(flow_direction),
            self.1.minima(flow_direction),
        )
    }

    fn lay<I>(&mut self, mut parent_hints: I) -> Self::Content
    where
        I: Iterator<Item = ParentHints>,
    {
        (
            self.0.lay(parent_hints.next().into_iter()),
            self.1.lay(parent_hints),
        )
    }
}
