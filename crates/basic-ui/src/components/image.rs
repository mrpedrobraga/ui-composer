pub use image;
use {
    crate::primitives::image_quad::ImageViewElementTerminal,
    image::{DynamicImage, GenericImageView},
    std::sync::Arc,
    ui_composer_core::{
        app::composition::layout::hints::{ChildHints, ParentHints},
        prelude::{Blueprint, LayoutItem},
    },
    ui_composer_math::prelude::{Rect, Size2},
    ui_composer_platform_tui::runner::{
        TerminalBlueprintResources, TerminalEnvironment,
    },
};

pub struct ImageView {
    // TODO: Find a replacement for #![no_std]!
    image: Arc<DynamicImage>,
    resized: Option<Size2>,
}

pub fn Image(image: Arc<DynamicImage>) -> ImageView {
    ImageView {
        image,
        resized: None,
    }
}

impl ImageView {
    pub fn with_resized(self, resized: Size2) -> Self {
        Self {
            resized: Some(resized),
            ..self
        }
    }
}

impl LayoutItem for ImageView {
    type Blueprint = ImageViewBlueprint;

    fn prepare(&mut self, _: ParentHints) -> ChildHints {
        let size = self.resized.unwrap_or_else(|| {
            let (w, h) = self.image.dimensions();
            Size2::<f32>::new(w as f32, h as f32)
        });

        ChildHints { minimum_size: size }
    }

    fn place(
        &mut self,
        parent_hints: ui_composer_core::app::composition::layout::hints::ParentHints,
    ) -> Self::Blueprint {
        ImageViewBlueprint {
            image: self.image.clone(),
            rect: parent_hints.rect,
        }
    }
}

pub struct ImageViewBlueprint {
    // TODO: Find a replacement for #![no_std]!
    pub image: Arc<DynamicImage>,
    pub rect: Rect,
}

impl Blueprint<TerminalEnvironment> for ImageViewBlueprint {
    type Element = ImageViewElementTerminal;

    fn make(self, _: &TerminalBlueprintResources) -> Self::Element {
        ImageViewElementTerminal::new(self.rect, self.image)
    }
}
