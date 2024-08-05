use futures_signals::signal::{Signal, SignalExt};
use vek::{Extent2, Rect};

pub mod flow;
pub mod functions;

use super::{
    node::{LiveUINode, UINode},
    react::{React, UISignalExt},
};

#[derive(Debug, Clone, Copy)]
pub struct LayoutHints {
    pub rect: Rect<f32, f32>,
}

/// An item that can be included in a layouting context.
pub trait LayoutItem: Send {
    type UINodeType: UINode;

    /// The size this component prefers to be at. It's usually it's minimum size.
    #[inline(always)]
    fn get_natural_size(&self) -> Extent2<f32>;

    /// Renders the content of this layout item with a specific rect.
    fn bake(&self, layout_hints: LayoutHints) -> Self::UINodeType;

    /// Creates a reactive node that re-bakes the layout item to fit a container that can change shape.
    fn bake_react<S>(
        self,
        size_signal: S,
    ) -> React<impl Signal<Item = Self::UINodeType>, Self::UINodeType>
    where
        S: Signal<Item = Extent2<f32>> + Send,
        Self: Sized + Send,
    {
        size_signal
            .map(move |new_size| {
                self.bake(LayoutHints {
                    rect: Rect::new(0.0, 0.0, new_size.w, new_size.h),
                })
            })
            .into_ui()
    }
}

pub struct Resizable<F: Send, T>
where
    F: Fn(LayoutHints) -> T,
{
    min_size: Extent2<f32>,
    factory: F,
}

impl<F, T> Resizable<F, T>
where
    F: Fn(LayoutHints) -> T + Send,
    T: LiveUINode,
{
    /// Creates a new resizable [`LayoutItem`] that redraws using this factory function.
    pub fn new(factory: F) -> Self {
        Self {
            min_size: Extent2::default(),
            factory,
        }
    }

    /// Consumes this [`Resizable`] and returns a similar one with the minimum size set.
    pub fn with_minimum_size(self, size: Extent2<f32>) -> Self {
        Self {
            min_size: size,
            ..self
        }
    }
}

impl<F: Send, T> LayoutItem for Resizable<F, T>
where
    F: Fn(LayoutHints) -> T,
    T: UINode,
{
    type UINodeType = T;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.min_size
    }

    fn bake(&self, layout_hints: LayoutHints) -> Self::UINodeType {
        (self.factory)(layout_hints)
    }
}

pub struct EmptyItem {}

impl LayoutItem for EmptyItem {
    type UINodeType = ();

    fn get_natural_size(&self) -> Extent2<f32> {
        Extent2::new(0.0, 0.0)
    }

    fn bake(&self, layout_hints: LayoutHints) -> Self::UINodeType {
        ()
    }
}

#[allow(non_snake_case)]
/// Creates an empty layout item.
pub fn Empty() -> EmptyItem {
    EmptyItem {}
}
