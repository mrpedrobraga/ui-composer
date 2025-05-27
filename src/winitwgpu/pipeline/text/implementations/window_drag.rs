use crate::{prelude::Drag, winitwgpu::pipeline::text::RenderText};

impl RenderText for Drag {
    fn push_text<'a>(
        &self,
        _buffer: &'a glyphon::Buffer,
        _bounds: glyphon::TextBounds,
        _container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
        // Nothing here!
    }
}
