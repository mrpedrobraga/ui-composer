use core::iter::{Chain, Once, once};
use ui_composer_core::app::composition::layout::{
    LayoutItem,
    hints::{ChildHints, ParentHints},
};
use ui_composer_geometry::flow::{
    CartesianFlow, CoordinateSystem as _, CurrentFlow, Flow, WritingFlow,
    arrangers::arrange_stretchy_rects_with_minimum_sizes_dirty_alloc,
};
use vek::{Extent2, Rect};

#[allow(non_snake_case)]
#[inline(always)]
pub fn flex<TItems>(items: TItems) -> FlexContainer<TItems>
where
    TItems: FlexItemList,
{
    FlexContainer {
        items,
        flow_direction: Flow::Writing(WritingFlow::WritingAxisForward),
    }
}

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

    fn prepare(&mut self, parent_hints: ParentHints) -> ChildHints {
        let flow_direction =
            self.flow_direction.as_cartesian(&parent_hints.current_flow);

        let mut min: Extent2<f32> = Extent2::zero();
        let mut nat: Extent2<f32> = Extent2::zero();

        for h in self.items.prepare(parent_hints) {
            match flow_direction {
                CartesianFlow::LeftToRight | CartesianFlow::RightToLeft => {
                    min.w += h.minimum_size.w;
                    min.h = min.h.max(h.minimum_size.h);
                    nat.w += h.natural_size.w;
                    nat.h = nat.h.max(h.natural_size.h);
                }
                CartesianFlow::TopToBottom | CartesianFlow::BottomToTop => {
                    min.w = min.w.max(h.minimum_size.w);
                    min.h += h.minimum_size.h;
                    nat.w = nat.w.max(h.natural_size.w);
                    nat.h += h.natural_size.h;
                }
            }
        }

        ChildHints {
            minimum_size: min,
            natural_size: nat,
        }
    }

    fn place(&mut self, parent_hints: ParentHints) -> Self::Blueprint {
        let flow_direction =
            self.flow_direction.as_cartesian(&parent_hints.current_flow);
        let minima = self.items.minima(flow_direction).collect::<Vec<_>>();
        let weights = self.items.weights().collect::<Vec<_>>();

        use CartesianFlow::*;
        let parent_size = match flow_direction {
            LeftToRight | RightToLeft => parent_hints.rect.w,
            TopToBottom | BottomToTop => parent_hints.rect.h,
        };

        let main_axis_sizes =
            arrange_stretchy_rects_with_minimum_sizes_dirty_alloc(
                parent_size,
                weights.as_slice(),
                minima.as_slice(),
                0.01,
            );

        let parent_hints_iter = lay_sizes(
            parent_hints,
            flow_direction,
            main_axis_sizes.into_iter(),
        );

        self.items.lay(parent_hints_iter)
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
                    container.rect.y + container.rect.h
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

pub struct FlexItem<T> {
    item: T,
    grow: f32,
    _hints_cache: ChildHints,
}

pub fn item<T>(item: T) -> FlexItem<T> {
    FlexItem {
        item,
        grow: 0.0,
        _hints_cache: ChildHints::default(),
    }
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

    fn prepare(
        &mut self,
        parent_hints: ParentHints,
    ) -> impl Iterator<Item = ChildHints>;
    fn weights(&self) -> Self::Weights;
    fn minima(&self, flow_direction: CartesianFlow) -> Self::Minima;
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

    fn prepare(
        &mut self,
        parent_hints: ParentHints,
    ) -> impl Iterator<Item = ChildHints> {
        let hints = self.item.prepare(parent_hints);
        self._hints_cache = hints;
        once(hints)
    }

    fn weights(&self) -> Once<f32> {
        once(self.grow)
    }

    fn minima(&self, flow_direction: CartesianFlow) -> Once<f32> {
        use CartesianFlow::*;
        match flow_direction {
            LeftToRight | RightToLeft => once(self._hints_cache.minimum_size.w),
            TopToBottom | BottomToTop => once(self._hints_cache.minimum_size.h),
        }
    }

    fn lay<I>(&mut self, mut hx: I) -> Self::Content
    where
        I: Iterator<Item = ParentHints>,
    {
        self.item.place(hx.next().unwrap())
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

    fn prepare(
        &mut self,
        parent_hints: ParentHints,
    ) -> impl Iterator<Item = ChildHints> {
        self.0
            .prepare(parent_hints)
            .chain(self.1.prepare(parent_hints))
    }

    fn weights(&self) -> Self::Weights {
        self.0.weights().chain(self.1.weights())
    }

    fn minima(&self, flow_direction: CartesianFlow) -> Self::Minima {
        self.0
            .minima(flow_direction)
            .chain(self.1.minima(flow_direction))
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
