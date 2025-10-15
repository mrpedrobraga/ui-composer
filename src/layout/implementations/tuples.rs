use crate::layout::LayoutItem;
use crate::layout::hints::ParentHints;
use vek::Extent2;

impl LayoutItem for () {
    type Content = ();

    fn get_natural_size(&self) -> Extent2<f32> {
        #[allow(deprecated)]
        self.get_minimum_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        Extent2::zero()
    }

    fn lay(&mut self, _layout_hints: ParentHints) -> Self::Content {}
}
