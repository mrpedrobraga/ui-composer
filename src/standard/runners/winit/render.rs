use crate::app::composition::elements::{Blueprint, Element};
use crate::runners::tui::Graphic;
use crate::runners::tui::render::RenderQuad;
use crate::runners::winit::runner::WinitEnvironment;

impl Blueprint<WinitEnvironment> for Graphic {
    type Element = Self;

    fn make(self, _: &WinitEnvironment) -> Self::Element {
        self
    }
}

impl Element<WinitEnvironment> for Graphic {
    type Effect = RenderQuad;

    fn effect(&self) -> Self::Effect {
        RenderQuad(self.rect, self.color)
    }
}
