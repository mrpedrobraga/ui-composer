use futures_signals::signal::{Signal, SignalExt};
use vek::{Extent2, Rect};

use super::{
    node::{LiveUINode, UINode},
    react::{React, UISignalExt},
};

/// An item that can be included in a layouting context.
pub trait LayoutItem {
    type UINodeType: UINode;

    /// The size this component prefers to be at. It's usually it's minimum size.
    fn get_natural_size(&self) -> Extent2<f32>;

    /// Renders the content of this layout item with a specific rect.
    fn bake(&self, rect: Rect<f32, f32>) -> Self::UINodeType;

    /// Creates a reactive node that re-bakes the layout item to fit a container that can change shape.
    fn bake_react<S>(
        self,
        size_signal: S,
    ) -> React<impl Signal<Item = Self::UINodeType>, Self::UINodeType>
    where
        S: Signal<Item = Extent2<f32>>,
        Self: Sized,
    {
        size_signal
            .map(move |new_size| self.bake(Rect::new(0.0, 0.0, new_size.w, new_size.h)))
            .into_ui()
    }
}

pub struct Resizable<F, T>
where
    F: Fn(Rect<f32, f32>) -> T,
{
    min_size: Extent2<f32>,
    factory: F,
}

impl<F, T> Resizable<F, T>
where
    F: Fn(Rect<f32, f32>) -> T,
    T: LiveUINode,
{
    pub fn new(min_size: Extent2<f32>, factory: F) -> Self {
        Self { min_size, factory }
    }
}

impl<F, T> LayoutItem for Resizable<F, T>
where
    F: Fn(Rect<f32, f32>) -> T,
    T: UINode,
{
    type UINodeType = T;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.min_size
    }

    fn bake(&self, rect: Rect<f32, f32>) -> Self::UINodeType {
        (self.factory)(rect)
    }
}
