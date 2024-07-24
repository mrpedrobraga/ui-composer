use vek::Aabr;

use crate::standard::render::UIFragment;

pub struct ReshapableFragment<T, F>
where
    T: UIFragment,
    F: FnOnce(Aabr<i32>) -> T,
{
    min_size: (i32, i32),
    factory: F,
}

impl<T, F> ReshapableFragment<T, F>
where
    T: UIFragment,
    F: FnOnce(Aabr<i32>) -> T,
{
    pub fn new(min_size: (i32, i32), factory: F) -> Self {
        Self { min_size, factory }
    }
}

/// An item that can produce a UIFragment given an AABB.
/// It delays the production of AABBs so that we can do layouting on it,
/// while still providing some internal hints such as its minimum size.
pub trait LayoutItem {
    fn get_natural_size(&self) -> (i32, i32);
    fn bake(self, aabb: Aabr<i32>) -> impl UIFragment;
}

impl<T, F> LayoutItem for ReshapableFragment<T, F>
where
    T: UIFragment,
    F: FnOnce(Aabr<i32>) -> T,
{
    fn get_natural_size(&self) -> (i32, i32) {
        self.min_size
    }

    fn bake(self, aabb: Aabr<i32>) -> impl UIFragment {
        (self.factory)(aabb)
    }
}
