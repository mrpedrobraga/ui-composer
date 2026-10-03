use crate::primitives::image_quad::RenderImageQuad;
pub use image;
use ui_composer_core::app::composition::elements::Element;

use {
    crate::primitives::image_quad::ImageViewElementTerminal,
    image::{DynamicImage, GenericImageView},
    std::sync::Arc,
    ui_composer_core::{
        app::composition::layout::hints::{ChildHints, ParentHints},
        prelude::{Blueprint, Ui},
    },
    ui_composer_math::prelude::{Rect, Size2},
    ui_composer_platform_tui::runner::{TerminalBlueprintResources, TerminalEnvironment},
};

#[pin_project::pin_project]
pub struct ImageView {
    image: Arc<DynamicImage>,
    #[pin]
    element: Option<ImageViewElementTerminal>,
    resized: Option<Size2>,
}

pub fn Image(image: Arc<DynamicImage>) -> ImageView {
    ImageView {
        image,
        element: None,
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

impl Ui<TerminalEnvironment> for ImageView {
    type Blueprint = ImageViewBlueprint;

    fn prepare(&mut self, _: ParentHints) -> ChildHints {
        let size = self.resized.unwrap_or_else(|| {
            let (w, h) = self.image.dimensions();
            Size2::<f32>::new(w as f32, h as f32)
        });

        ChildHints { minimum_size: size }
    }

    fn place(&mut self, parent_hints: ParentHints, resources: &TerminalBlueprintResources) {
        if let Some(element) = &mut self.element {
            element.update(
                ImageViewBlueprint {
                    image: self.image.clone(),
                    rect: parent_hints.rect,
                },
                resources,
            );
        }
    }

    fn effect(&self) -> RenderImageQuad {
        self.element
            .effect()
            // TODO: Not use ZERO here.
            // Unsure what to do, really.
            .unwrap_or(RenderImageQuad(Rect::ZERO, self.image.clone()))
    }

    async fn propagate(&mut self, _: &mut ui_composer_input::event::Event) -> bool {
        false
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
        resources: &TerminalBlueprintResources,
        _: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        let this = self.project();
        this.element.poll_change(cx, resources)
    }
}

pub struct ImageViewBlueprint {
    // TODO: Find a replacement for #![no_std]!
    pub image: Arc<DynamicImage>,
    pub rect: Rect,
}

impl Blueprint<TerminalEnvironment> for ImageViewBlueprint {
    type Output = ImageViewElementTerminal;

    fn make(self, _: &TerminalBlueprintResources) -> Self::Output {
        ImageViewElementTerminal::new(self.rect, self.image)
    }
}
