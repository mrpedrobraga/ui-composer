use vek::{Extent2, Rect, Rgb};

use crate::ui::{graphics::Primitive, layout::LayoutItem, node::UINode};

/// A node that describes the existence of a View in the UI tree.
///
/// A "View" is a render indirection. Primitives will render to the view,
/// and the view will render to its parent within the bounds of a certain rect.
///
/// There are two relevant Rects when it comes to Views. Since View is a LayoutItem,
/// it will be rendered to a rect on its parent. But there's also a Rect that exists
pub struct ViewNode<T: UINode> {
    min_size: Extent2<f32>,
    content: T,
}

impl<T> LayoutItem for ViewNode<T>
where
    T: UINode,
{
    type UINodeType = Primitive;

    fn get_natural_size(&self) -> vek::Extent2<f32> {
        self.min_size
    }

    fn bake(&self, rect: Rect<f32, f32>) -> Self::UINodeType {
        /// TODO: Bind this primitive to the texture that
        /// the contents of the view render to.
        Primitive::rect(rect, Rgb::new(0.0, 0.0, 0.0))
    }
}

/// Creates a new view as the render target for the nodes inside.
#[allow(non_snake_case)]
pub fn View<T>(min_size: Extent2<f32>, item: T) -> ViewNode<impl UINode>
where
    T: UINode,
{
    ViewNode {
        min_size,
        content: item,
    }
}
