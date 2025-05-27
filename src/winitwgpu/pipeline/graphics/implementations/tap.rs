use crate::{
    prelude::Tap,
    state::Effect,
    winitwgpu::pipeline::graphics::{graphic::Graphic, RenderGraphic, RenderGraphicDescriptor},
};
use vek::Rect;

impl<A> RenderGraphicDescriptor for Tap<A>
where
    A: Effect + Send + Sync,
{
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None // Some(self.area))
    }
}
impl<A: Effect + Send + Sync> RenderGraphic for Tap<A> {
    fn write_quads(&self, _quad_buffer: &mut [Graphic]) {
        /* Maybe push something here in Debug mode? */
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}
