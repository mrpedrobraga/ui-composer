use crate::app::composition::layout::LayoutItem;
use crate::app::composition::layout::hints::ParentHints;
use vek::Extent2;

impl LayoutItem for () {
    type Blueprint = ();

    fn get_natural_size(&self) -> Extent2<f32> {
        self.get_minimum_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        Extent2::zero()
    }

    fn lay(&mut self, _layout_hints: ParentHints) -> Self::Blueprint {}
}

impl<A, B> LayoutItem for (A, B)
where
    A: LayoutItem,
    B: LayoutItem,
{
    type Blueprint = (A::Blueprint, B::Blueprint);

    fn get_natural_size(&self) -> Extent2<f32> {
        let a = self.0.get_natural_size();
        let b = self.1.get_natural_size();
        Extent2::new(a.w.max(b.w), a.w.max(b.h))
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        let a = self.0.get_minimum_size();
        let b = self.1.get_minimum_size();
        Extent2::new(a.w.max(b.w), a.w.max(b.h))
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::Blueprint {
        (self.0.lay(parent_hints), self.1.lay(parent_hints))
    }
}

impl<A> LayoutItem for Box<A>
where
    A: LayoutItem + ?Sized,
{
    type Blueprint = A::Blueprint;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.as_ref().get_natural_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.as_ref().get_minimum_size()
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::Blueprint {
        self.as_mut().lay(parent_hints)
    }
}
