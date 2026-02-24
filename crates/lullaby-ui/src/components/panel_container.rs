use {
    crate::{components::UI, list_internal},
    ui_composer_basic_ui::primitives::graphic::Graphic,
    ui_composer_core::app::composition::layout::{ItemBox, Resizable as _},
    vek::Rgba,
};

static SURFACE_COLOR: Rgba<f32> = Rgba::new(255.0, 253.0, 248.0, 255.0);
#[allow(unused)]
static SURFACE_COLOR_2: Rgba<f32> = Rgba::new(255.0, 241.0, 231.0, 255.0);

pub fn PanelContainer(mut child: impl UI) -> impl UI {
    let min_size = child.get_minimum_size();

    ItemBox::new(move |hx| {
        let rect = Graphic::new(hx.rect, SURFACE_COLOR / 255.0);
        let c = child.lay(hx);

        list_internal![rect, c]
    })
    .with_minimum_size(min_size)
}
