use vek::{Extent2, Rect, Rgb};

use crate::ui::{
    graphics::Graphic,
    layout::{LayoutItem, ParentHints},
    node::ItemDescriptor,
};

/// A node that describes the existence of a View in the UI tree.
///
/// A "View" is a render indirection. Primitives will render to the view,
/// and the view will render to its parent within the bounds of a certain rect.
///
/// There are two relevant Rects when it comes to Views. Since View is a LayoutItem,
/// it will be rendered to a rect on its parent. But there's also a Rect that exists
pub struct ViewNode<T: ItemDescriptor> {
    min_size: Extent2<f32>,
    content: T,
}

/// TODO: A View should create a single primitive bound to a texture that the contents draw onto.
impl<T> LayoutItem for ViewNode<T>
where
    T: ItemDescriptor,
{
    type UINodeType = Graphic;

    fn get_natural_size(&self) -> vek::Extent2<f32> {
        self.min_size
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.min_size
    }

    fn lay(&self, layout_hints: ParentHints) -> Self::UINodeType {
        Graphic::new(layout_hints.rect, Rgb::green())
    }
}

/// Creates a new view as the render target for the nodes inside.
#[allow(non_snake_case)]
pub fn View<T>(min_size: Extent2<f32>, item: T) -> ViewNode<impl ItemDescriptor>
where
    T: ItemDescriptor,
{
    ViewNode {
        min_size,
        content: item,
    }
}
