use vek::{Extent2, Rect};

use super::node::UINode;

/// An item that can be included in a layouting context.
pub trait LayoutItem {
    type UINodeType: UINode;

    /// The size this component prefers to be at. It's usually it's minimum size.
    fn get_natural_size(&self) -> Extent2<f32>;

    fn bake(&self, rect: Rect<f32, f32>) -> Self::UINodeType;
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
    T: UINode,
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
