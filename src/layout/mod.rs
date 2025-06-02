use crate::app::primitives::PrimitiveDescriptor;
use crate::prelude::process::React;
pub use flow::CoordinateSystemProvider;
use std::marker::PhantomData;
use {
    crate::prelude::flow::CartesianFlowDirection,
    futures_signals::signal::{Signal, SignalExt},
    vek::{Extent2, Mat3, Rect, Vec2},
};

pub mod flow;

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
#[diagnostic::on_unimplemented(
    message = "{Self} is not a `LayoutItem` thus can not be used...",
    label = "...in this context...",
    note = "You can use `ResizableItem` to use graphics/input primitives as layout items."
)]
#[must_use = "layout items need to be put in a layout context to be used."]
pub trait LayoutItem: Send {
    type Content;

    /// The size this component prefers to be at. It's usually its minimum size.
    ///
    /// DEPRECATED: Instead of a callback, [Self::lay] will have a "natural size"
    /// reverse [Signal] in its child hints.
    #[deprecated]
    fn get_natural_size(&self) -> Extent2<f32>;

    /// The size this component prefers to be at. It's usually its minimum size.
    ///
    /// DEPRECATED: Instead of a callback, [Self::lay] will have a "minimum size"
    /// reverse [Signal] in its child hints.
    #[deprecated]
    fn get_minimum_size(&self) -> Extent2<f32>;

    /// Renders the content of this layout item with a specific rect.
    fn lay(&mut self, parent_hints: ParentHints) -> Self::Content;

    /// Creates a reactive node that re-bakes the layout item to fit a container that can change shape.
    fn lay_reactive<S>(
        mut self,
        size_signal: S,
        parent_hints: ParentHints,
    ) -> React<impl Signal<Item = Self::Content>>
    where
        S: Signal<Item = Extent2<f32>> + Send,
        Self: Sized + Send,
    {
        React(size_signal.map(move |new_size| {
            self.lay(ParentHints {
                rect: Rect::new(0.0, 0.0, new_size.w, new_size.h),
                ..parent_hints
            })
        }))
    }
}

pub struct ResizableItem<F, A, Resources>
where
    F: Send + FnMut(ParentHints) -> A,
{
    hints: ChildHints,
    factory: F,
    __resources: PhantomData<fn() -> Resources>,
}

pub trait Resizable: LayoutItem {
    /// Consumes this [`ResizableItem`] and returns a similar one with the minimum size set.
    fn with_minimum_size(self, min_size: Extent2<f32>) -> Self;
}

impl<F, T, Res> ResizableItem<F, T, Res>
where
    F: FnMut(ParentHints) -> T + Send,
    T: PrimitiveDescriptor<Res>,
{
    /// Creates a new resizable [`LayoutItem`] that redraws using this factory function.
    pub fn new(factory: F) -> Self {
        Self {
            hints: ChildHints::default(),
            factory,
            __resources: PhantomData,
        }
    }
}

impl<F, T, Res> Resizable for ResizableItem<F, T, Res>
where
    F: Send + FnMut(ParentHints) -> T,
{
    fn with_minimum_size(self, min_size: Extent2<f32>) -> Self {
        Self {
            hints: ChildHints { min_size },
            ..self
        }
    }
}

impl<F: Send, T, Res> LayoutItem for ResizableItem<F, T, Res>
where
    F: FnMut(ParentHints) -> T,
{
    type Content = T;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.get_minimum_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.hints.min_size
    }

    fn lay(&mut self, layout_hints: ParentHints) -> Self::Content {
        (self.factory)(layout_hints)
    }
}

impl LayoutItem for () {
    type Content = ();

    fn get_natural_size(&self) -> Extent2<f32> {
        self.get_minimum_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        Extent2::zero()
    }

    fn lay(&mut self, _layout_hints: ParentHints) -> Self::Content {}
}
