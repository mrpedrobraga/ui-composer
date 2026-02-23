use {
    crate::{components::UI, primitives::text::Text},
    ui_composer_core::app::composition::layout::{ItemBox, Resizable as _},
    vek::{Extent2, Rgba},
};

static TEXT_COLOR: Rgba<f32> = Rgba::new(156.0, 78.0, 10.0, 255.0);

pub fn Label(string: impl ToString) -> impl UI {
    let string = string.to_string();

    ItemBox::new(move |hx| {
        Text()
            .with_text(string.clone())
            .with_rect(hx.rect)
            .with_color(TEXT_COLOR / 255.0)
    })
    .with_minimum_size(Extent2::new(64.0, 1.0))
}
