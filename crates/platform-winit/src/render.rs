/*
impl Blueprint<WinitEnvironment> for Graphic {
    type Element = Self;

    fn make(self, _: &WinitEnvironment) -> Self::Element {
        self
    }
}

impl Element<WinitEnvironment> for Graphic {
    type Effect<'a> = RenderQuad;

    fn effect(&self) -> Self::Effect<'_> {
        RenderQuad(self.rect, self.color)
    }
}
*/
