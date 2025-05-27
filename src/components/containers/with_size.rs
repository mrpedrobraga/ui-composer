use {
    crate::prelude::{LayoutItem, ParentHints},
    vek::Extent2,
};

/// A container that scales its single item to a bigger size.
/// You **can not** make the minimum size _lower_ than the original, however.
pub fn WithSize<A>(suggested_size: Extent2<f32>, item: A) -> WithSizeContainer<A>
where
    A: LayoutItem,
{
    WithSizeContainer {
        suggested_size,
        item,
    }
}

pub struct WithSizeContainer<A>
where
    A: LayoutItem,
{
    suggested_size: Extent2<f32>,
    item: A,
}

impl<A> LayoutItem for WithSizeContainer<A>
where
    A: LayoutItem,
{
    type UIItemType = A::UIItemType;

    fn get_natural_size(&self) -> Extent2<f32> {
        let inner_size = self.item.get_natural_size();
        Extent2::new(
            f32::max(self.suggested_size.w, inner_size.w),
            f32::max(self.suggested_size.h, inner_size.h),
        )
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.item.get_minimum_size()
    }

    fn lay(&mut self, layout_hints: ParentHints) -> Self::UIItemType {
        self.item.lay(layout_hints)
    }
}
