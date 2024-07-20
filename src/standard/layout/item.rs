use crate::geometry::aabb::AABB;
use crate::standard::render_stack::UIFragment;

pub struct ReshapableFragment<T, F>
where
    T: UIFragment,
    F: FnOnce(AABB) -> T,
{
    min_size: (i32, i32),
    factory: F,
}

impl<T, F> ReshapableFragment<T, F>
where
    T: UIFragment,
    F: FnOnce(AABB) -> T,
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
    fn bake(self, aabb: AABB) -> impl UIFragment;
}

impl<T, F> LayoutItem for ReshapableFragment<T, F>
where
    T: UIFragment,
    F: FnOnce(AABB) -> T,
{
    fn get_natural_size(&self) -> (i32, i32) {
        self.min_size
    }

    fn bake(self, aabb: AABB) -> impl UIFragment {
        (self.factory)(aabb)
    }
}
