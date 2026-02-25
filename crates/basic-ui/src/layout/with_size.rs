use {
    ui_composer_core::app::composition::layout::{
        LayoutItem, hints::ParentHints,
    },
    vek::Extent2,
};

pub struct WithSizeContainer<A>
where
    A: LayoutItem,
{
    suggested_size: Extent2<f32>,
    item: A,
}

/// A container that scales its single item to a bigger size.
/// You **can not** make the minimum size _lower_ than the original, however.
pub fn with_size<A>(item: A) -> WithSizeContainer<A>
where
    A: LayoutItem,
{
    WithSizeContainer {
        suggested_size: Extent2::zero(),
        item,
    }
}

impl<A> WithSizeContainer<A>
where
    A: LayoutItem,
{
    pub fn with_size(self, suggested_size: Extent2<f32>) -> Self {
        Self {
            suggested_size,
            ..self
        }
    }
}

impl<A> LayoutItem for WithSizeContainer<A>
where
    A: LayoutItem,
{
    type Blueprint = A::Blueprint;

    fn get_natural_size(&self) -> Extent2<f32> {
        let inner_size = self.item.get_natural_size();
        Extent2::new(
            f32::max(self.suggested_size.w, inner_size.w),
            f32::max(self.suggested_size.h, inner_size.h),
        )
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        let inner_size = self.item.get_minimum_size();
        Extent2::new(
            f32::max(self.suggested_size.w, inner_size.w),
            f32::max(self.suggested_size.h, inner_size.h),
        )
    }

    fn place(&mut self, layout_hints: ParentHints) -> Self::Blueprint {
        self.item.place(layout_hints)
    }
}
