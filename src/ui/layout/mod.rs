use super::node::{ItemDescriptor, UIItem};
use crate::state::process::{SignalProcessor, UISignalExt};
use futures_signals::signal::{Signal, SignalExt};
use vek::{Extent2, Rect};

pub mod flow;
pub mod functions;

#[derive(Debug, Clone, Copy)]
pub struct ParentHints {
    pub rect: Rect<f32, f32>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ChildHints {
    min_size: Extent2<f32>,
}

/// An item that can be included in a laying out context.
pub trait LayoutItem: Send {
    type UINodeType: ItemDescriptor;

    /// The size this component prefers to be at. It's usually its minimum size.
    #[inline(always)]
    fn get_natural_size(&self) -> Extent2<f32>;

    /// The size this component prefers to be at. It's usually its minimum size.
    #[inline(always)]
    fn get_minimum_size(&self) -> Extent2<f32>;

    /// Renders the content of this layout item with a specific rect.
    fn lay(&mut self, parent_hints: ParentHints) -> Self::UINodeType;

    /// Creates a reactive node that re-bakes the layout item to fit a container that can change shape.
    fn lay_reactive<S>(
        mut self,
        size_signal: S,
    ) -> SignalProcessor<impl Signal<Item = Self::UINodeType>, Self::UINodeType>
    where
        S: Signal<Item = Extent2<f32>> + Send,
        Self: Sized + Send,
    {
        size_signal
            .map(move |new_size| {
                self.lay(ParentHints {
                    rect: Rect::new(0.0, 0.0, new_size.w, new_size.h),
                })
            })
            .process()
    }
}

pub struct Resizable<F: Send, T>
where
    F: FnMut(ParentHints) -> T,
{
    hints: ChildHints,
    factory: F,
}

impl<F, T> Resizable<F, T>
where
    F: FnMut(ParentHints) -> T + Send,
    T: UIItem,
{
    /// Creates a new resizable [`LayoutItem`] that redraws using this factory function.
    pub fn new(factory: F) -> Self {
        Self {
            hints: ChildHints::default(),
            factory,
        }
    }

    /// Consumes this [`Resizable`] and returns a similar one with the minimum size set.
    pub fn with_minimum_size(self, min_size: Extent2<f32>) -> Self {
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

impl<F: Send, T> LayoutItem for Resizable<F, T>
where
    F: FnMut(ParentHints) -> T,
    T: ItemDescriptor,
{
    type UINodeType = T;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.get_minimum_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.hints.min_size
    }

    fn lay(&mut self, layout_hints: ParentHints) -> Self::UINodeType {
        (self.factory)(layout_hints)
    }
}

impl LayoutItem for () {
    type UINodeType = ();

    fn get_natural_size(&self) -> Extent2<f32> {
        self.get_minimum_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        Extent2::new(0.0, 0.0)
    }

    fn lay(&mut self, layout_hints: ParentHints) -> Self::UINodeType {
        ()
    }
}
