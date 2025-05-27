use crate::app::node::{AppItem, AppItemDescriptor};
use crate::gpu::render_target::Render;
use crate::prelude::flow::CartesianFlowDirection;
use crate::state::process::{SignalProcessor, UISignalExt};
use futures_signals::signal::{Signal, SignalExt};
use vek::{Extent2, Mat3, Rect, Vec2};

pub mod flow;

pub use flow::CoordinateSystemProvider;

#[derive(Debug, Clone, Copy)]
pub struct ParentHints {
    pub rect: Rect<f32, f32>,
    pub current_flow_direction: CartesianFlowDirection,
    pub current_cross_flow_direction: CartesianFlowDirection,
    pub current_writing_flow_direction: CartesianFlowDirection,
    pub current_writing_cross_flow_direction: CartesianFlowDirection,
}

impl ParentHints {
    pub fn writing_axis(&self) -> Vec2<f32> {
        self.current_writing_flow_direction.get_axes(self)
    }

    pub fn writing_cross_axis(&self) -> Vec2<f32> {
        self.current_writing_cross_flow_direction.get_axes(self)
    }

    pub fn writing_origin(&self) -> Vec2<f32> {
        self.current_writing_flow_direction.get_origin(self)
    }

    pub fn writing_cross_origin(&self) -> Vec2<f32> {
        self.current_writing_cross_flow_direction.get_origin(self)
    }

    pub fn writing_coordinate_system(&self) -> Mat3<f32> {
        let wo = self.writing_origin();
        let wx = self.writing_axis();
        let wy = self.writing_cross_axis();

        Mat3::new(wx.x, wx.y, 0.0, wy.x, wy.y, 0.0, wo.x, wo.y, 1.0)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ChildHints {
    min_size: Extent2<f32>,
}

/// An item that can be included in a laying out context.
pub trait LayoutItem: Send {
    type UIItemType: Render + AppItemDescriptor;

    /// The size this component prefers to be at. It's usually its minimum size.
    #[inline(always)]
    fn get_natural_size(&self) -> Extent2<f32>;

    /// The size this component prefers to be at. It's usually its minimum size.
    #[inline(always)]
    fn get_minimum_size(&self) -> Extent2<f32>;

    /// Renders the content of this layout item with a specific rect.
    fn lay(&mut self, parent_hints: ParentHints) -> Self::UIItemType;

    /// Creates a reactive node that re-bakes the layout item to fit a container that can change shape.
    fn lay_reactive<S>(
        mut self,
        size_signal: S,
        parent_hints: ParentHints,
    ) -> SignalProcessor<impl Signal<Item = Self::UIItemType>, Self::UIItemType>
    where
        S: Signal<Item = Extent2<f32>> + Send,
        Self: Sized + Send, <Self as LayoutItem>::UIItemType: AppItem
    {
        size_signal
            .map(move |new_size| {
                self.lay(ParentHints {
                    rect: Rect::new(0.0, 0.0, new_size.w, new_size.h),
                    ..parent_hints
                })
            })
            .process()
    }
}

pub struct ResizableItem<F: Send, T>
where
    F: FnMut(ParentHints) -> T,
{
    hints: ChildHints,
    factory: F,
}

pub trait Resizable: LayoutItem {
    fn with_minimum_size(self, min_size: Extent2<f32>) -> Self;
}

impl<F, T> ResizableItem<F, T>
where
    F: FnMut(ParentHints) -> T + Send,
    T: AppItem,
{
    /// Creates a new resizable [`LayoutItem`] that redraws using this factory function.
    pub fn new(factory: F) -> Self {
        Self {
            hints: ChildHints::default(),
            factory,
        }
    }
}

impl<F, T> Resizable for ResizableItem<F, T>
where
    F: FnMut(ParentHints) -> T + Send,
    T: Render + AppItemDescriptor
{
    /// Consumes this [`ResizableItem`] and returns a similar one with the minimum size set.
    fn with_minimum_size(self, min_size: Extent2<f32>) -> Self {
        let child_hints = self.hints;

        Self {
            hints: ChildHints {
                min_size,
                ..child_hints
            },
            ..self
        }
    }
}

impl<F: Send, T> LayoutItem for ResizableItem<F, T>
where
    F: FnMut(ParentHints) -> T,
    T: Render + AppItemDescriptor
{
    type UIItemType = T;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.get_minimum_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.hints.min_size
    }

    fn lay(&mut self, layout_hints: ParentHints) -> Self::UIItemType {
        (self.factory)(layout_hints)
    }
}

impl LayoutItem for () {
    type UIItemType = ();

    fn get_natural_size(&self) -> Extent2<f32> {
        self.get_minimum_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        Extent2::new(0.0, 0.0)
    }

    fn lay(&mut self, layout_hints: ParentHints) -> Self::UIItemType {
        ()
    }
}
