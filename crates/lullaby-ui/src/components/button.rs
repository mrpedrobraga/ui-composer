use {
    crate::{components::UI, list_internal, primitives::graphic::Graphic},
    ui_composer_basic_ui::items::{Hover, Tap},
    ui_composer_core::app::composition::{
        effects::signal::SignalReactExt as _,
        layout::{ItemBox, Resizable as _, hints::ParentHints},
    },
    ui_composer_geometry::RectExt as _,
    ui_composer_state::{effect::Effect, futures_signals::signal::Mutable},
    vek::{Extent2, Rgba},
};

static BUTTON_COLOR: Rgba<f32> = Rgba::new(255.0, 217.0, 179.0, 255.0);
static BUTTON_COLOR_HOVER: Rgba<f32> = Rgba::new(235.0, 189.0, 143.0, 255.0);

/// This is what `text_color` gets overriden with in a cascading context.
#[allow(unused)]
static BUTTON_TEXT_COLOR: Rgba<f32> = Rgba::new(175.0, 90.0, 16.0, 255.0);

/// A simple button which can be clicked to trigger some `effect`.
/// The button supports a `label` component which will be displayed inside the button
pub fn Button(mut label: impl UI, effect: impl Effect + 'static) -> impl UI {
    let is_hovered: Mutable<bool> = Mutable::default();

    ItemBox::new(move |hx| {
        let hover = Hover::new(hx.rect, is_hovered.clone());
        let tap = Tap::new(hx.rect, effect.clone());

        let rect = is_hovered
            .signal_ref(move |is_hovered| {
                if *is_hovered {
                    Graphic::new(hx.rect, BUTTON_COLOR_HOVER / 255.0)
                } else {
                    Graphic::new(hx.rect, BUTTON_COLOR / 255.0)
                }
            })
            .react();
        let label = label.lay(ParentHints {
            rect: hx.rect.expand_from_center(-1.0, -1.0, -1.0, -1.0),
            ..hx
        });

        list_internal![hover, tap, rect, label]
    })
    .with_minimum_size(Extent2::new(15.0, 3.0))
}
