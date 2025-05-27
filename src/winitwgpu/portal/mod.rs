#![allow(unused)]
use {
    super::pipeline::graphics::graphic::Graphic,
    vek::{Extent2, Rgb},
};

use crate::ui::layout::{LayoutItem, ParentHints};

/// A node that describes the existence of a Portal in the UI tree.
///
/// A [PortalNode] is a render indirection. Primitives will render to the view,
/// and the view will render to its parent within the bounds of a certain rect.
///
/// There are two relevant Rects when it comes to Portal. Since Portal is a LayoutItem,
/// it will be rendered to a rect on its parent. But there's also a Rect that exists
pub struct PortalNode<A> {
    min_size: Extent2<f32>,
    content: A,
}

/// TODO: A View should create a single primitive bound to a texture that the contents draw onto.
impl<T: Send> LayoutItem for PortalNode<T> {
    type UIItemType = Graphic;

    fn get_natural_size(&self) -> vek::Extent2<f32> {
        self.min_size
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.min_size
    }

    fn lay(&mut self, layout_hints: ParentHints) -> Self::UIItemType {
        Graphic::new(layout_hints.rect, Rgb::green())
    }
}

/// Creates a new view as the render target for the nodes inside.
#[allow(non_snake_case)]
pub fn View<T>(min_size: Extent2<f32>, item: T) -> PortalNode<T> {
    PortalNode {
        min_size,
        content: item,
    }
}
