use {
    crate::{
        prelude::Drag,
        winitwgpu::pipeline::graphics::{graphic::Graphic, RenderGraphic, RenderGraphicDescriptor},
    },
    vek::Rect,
};

impl RenderGraphicDescriptor for Drag {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None // Some(self.area))
    }
}

impl RenderGraphic for Drag {
    fn write_quads(&self, _quad_buffer: &mut [Graphic]) {
        /* Maybe push something here in Debug mode? */
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}
