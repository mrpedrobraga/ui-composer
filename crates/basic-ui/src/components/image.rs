pub use image;
use {
    crate::primitives::image_quad::ImageViewElementTerminal,
    image::{DynamicImage, GenericImageView},
    std::sync::Arc,
    ui_composer_core::prelude::{Blueprint, LayoutItem},
    ui_composer_platform_tui::runner::TerminalEnvironment,
    vek::{Extent2, Rect},
};

pub struct ImageView {
    // TODO: Find a replacement for #![no_std]!
    image: Arc<DynamicImage>,
    resized: Option<Extent2<f32>>,
}

pub fn Image(image: DynamicImage) -> ImageView {
    ImageView {
        image: Arc::new(image),
        resized: None,
    }
}

impl ImageView {
    pub fn with_resized(self, resized: Extent2<f32>) -> Self {
        Self {
            resized: Some(resized),
            ..self
        }
    }
}

impl LayoutItem for ImageView {
    type Blueprint = ImageViewBlueprint;

    fn get_natural_size(&self) -> vek::Extent2<f32> {
        self.resized.unwrap_or_else(|| {
            let (w, h) = self.image.dimensions();
            Extent2::new(w, h).as_()
        })
    }

    fn get_minimum_size(&self) -> vek::Extent2<f32> {
        self.resized.unwrap_or_else(|| {
            let (w, h) = self.image.dimensions();
            Extent2::new(w, h).as_()
        })
    }

    fn lay(
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
    pub rect: Rect<f32, f32>,
}

impl Blueprint<TerminalEnvironment> for ImageViewBlueprint {
    type Element = ImageViewElementTerminal;

    fn make(self, _: &TerminalEnvironment) -> Self::Element {
        ImageViewElementTerminal::new(self.rect, self.image)
    }
}
