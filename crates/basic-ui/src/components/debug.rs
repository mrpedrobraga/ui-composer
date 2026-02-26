use ui_composer_core::{
    app::composition::layout::hints::{ChildHints, ParentHints},
    prelude::LayoutItem,
};
use ui_composer_math::prelude::{Size2, Srgba};

use crate::primitives::graphic::Graphic;

#[derive(Default)]
pub struct ColorBox {
    size: Size2,
    color: Srgba,
}

#[allow(non_snake_case)]
pub fn ColorBox() -> ColorBox {
    ColorBox {
        ..Default::default()
    }
}
impl ColorBox {
    pub fn with_color(self, color: Srgba) -> Self {
        Self { color, ..self }
    }

    pub fn with_size(self, size: Size2) -> Self {
        Self { size, ..self }
    }
}

impl LayoutItem for ColorBox {
    type Blueprint = Graphic;

    fn prepare(&mut self, _: ParentHints) -> ChildHints {
        ChildHints {
            minimum_size: self.size,
        }
    }

    fn place(&mut self, parent_hints: ParentHints) -> Self::Blueprint {
        Graphic {
            rect: parent_hints.rect,
            color: self.color,
        }
    }
}
