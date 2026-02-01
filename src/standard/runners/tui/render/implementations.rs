use vek::{Rect, Rgba};
use crate::runners::tui::render::{Canvas, RenderTui};

impl<A, B> RenderTui for (A, B) where A: RenderTui, B: RenderTui {
    fn draw<C>(&self, canvas: &mut C, rect: Rect<u16, u16>)
    where
        C: Canvas<Pixel=Rgba<u8>>
    {
        self.0.draw(canvas, rect);
        self.1.draw(canvas, rect);
    }
}