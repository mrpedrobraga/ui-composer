use ui_composer_core::prelude::LayoutItem;
use vek::{Extent2, Rgba};

use crate::primitives::graphic::Graphic;

#[derive(Default)]
pub struct DebugSquare {
    size: Extent2<f32>,
    color: Rgba<f32>,
}

#[allow(non_snake_case)]
pub fn ColorBox() -> DebugSquare {
    DebugSquare {
        ..Default::default()
    }
}
impl DebugSquare {
    pub fn with_color(self, color: Rgba<f32>) -> Self {
        Self { color, ..self }
    }

    pub fn with_size(self, size: Extent2<f32>) -> Self {
        Self { size, ..self }
    }
}

impl LayoutItem for DebugSquare {
    type Blueprint = Graphic;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.size
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.size
    }

    fn place(
        &mut self,
        parent_hints: ui_composer_core::app::composition::layout::hints::ParentHints,
    ) -> Self::Blueprint {
        Graphic {
            rect: parent_hints.rect,
            color: self.color,
        }
    }
}
