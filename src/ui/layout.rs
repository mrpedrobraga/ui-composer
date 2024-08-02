use vek::{Extent2, Rect};

use super::node::UINode;

/// An item that can be included in a layouting context.
pub trait LayoutItem {
    type UINodeType: UINode;

    /// The size this component prefers to be at. It's usually it's minimum size.
    fn get_natural_size(&self) -> Extent2<f32>;

    fn bake(&self, rect: Rect<f32, f32>) -> Self::UINodeType;
}
