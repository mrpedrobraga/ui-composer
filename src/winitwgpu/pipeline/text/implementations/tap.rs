use crate::{prelude::Tap, state::Effect, winitwgpu::pipeline::text::RenderText};

impl<A: Effect + Send + Sync> RenderText for Tap<A> {
    fn push_text<'a>(
        &self,
        _buffer: &'a glyphon::Buffer,
        _bounds: glyphon::TextBounds,
        _container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
        // Nothing here!
    }
}
