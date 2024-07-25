use vek::{Extent2, Rect};

use crate::standard::render::UIFragment;

/// A UI Fragment that can be reshaped by a parent.
#[derive(Clone)]
pub struct ResizableItem<T, F>
where
    T: UIFragment + 'static,
    F: 'static + Fn(Rect<f32, f32>) -> T,
{
    min_size: Extent2<f32>,
    factory: F,
}

impl<T, F> ResizableItem<T, F>
where
    T: UIFragment + 'static,
    F: 'static + Fn(Rect<f32, f32>) -> T,
{
    pub fn new(min_size: Extent2<f32>, factory: F) -> Self {
        Self { min_size, factory }
    }
}

/// An item that can produce a UIFragment given an AABB.
/// It delays the production of AABBs so that we can do layouting on it,
/// while still providing some internal hints such as its minimum size.
pub trait LayoutItem {
    type BakeResult: UIFragment;
    fn get_natural_size(&self) -> Extent2<f32>;
    fn bake(&self, rect: Rect<f32, f32>) -> Self::BakeResult;
}

impl<T, F> LayoutItem for ResizableItem<T, F>
where
    T: UIFragment + 'static,
    F: (Fn(Rect<f32, f32>) -> T) + 'static,
{
    type BakeResult = T;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.min_size
    }

    fn bake(&self, rect: Rect<f32, f32>) -> T {
        let val: T = (self.factory)(rect);
        val
    }
}
