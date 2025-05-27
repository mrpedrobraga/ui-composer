//MARK: Input

use crate::{prelude::Hover, winitwgpu::pipeline::text::RenderText};

impl RenderText for Hover {
    fn push_text<'a>(
        &self,
        _buffer: &'a glyphon::Buffer,
        _bounds: glyphon::TextBounds,
        _container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
        // Nothing here!
    }
}
