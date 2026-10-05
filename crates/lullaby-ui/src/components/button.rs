use ::ui_composer_basic_ui::layout::{center};
use ::ui_composer_core::app::composition::{
    effects::signal::SignalExt as _,
    elements::{Blueprint, Environment},
    CompatibleWith,
};
use {
    crate::list_internal,
    ui_composer_basic_ui::{interaction::Tap, primitives::graphic::Graphic},
    ui_composer_core::app::composition::layout::{Canvas, Resizable as _},
    ui_composer_math::prelude::{Size2, Srgba},
    ui_composer_state::{effect::Effect, futures_signals::signal::Mutable},
};

#[allow(unused)]
static BUTTON_COLOR: Srgba = Srgba::new(255.0, 217.0, 179.0, 255.0);
#[allow(unused)]
static BUTTON_COLOR_HOVER: Srgba = Srgba::new(235.0, 189.0, 143.0, 255.0);

/// This is what `text_color` gets overriden with in a cascading context.
#[allow(unused)]
static BUTTON_TEXT_COLOR: Srgba = Srgba::new(175.0, 90.0, 16.0, 255.0);

/// A simple button which can be clicked to trigger some `effect`.
/// The button supports a `label` component which will be displayed inside the button
pub fn Button<Env: Environment, U: CompatibleWith<Env>, E: Effect + 'static>(
    label: U,
    effect: E,
) -> impl CompatibleWith<Env>
where
    Graphic: Blueprint<Env>,
    Tap<E>: Blueprint<Env>,
{
    let is_hovered: Mutable<bool> = Mutable::default();

    /* TODO: Use a single layout component for the bg and tap area instead of many. */

    let _bg = is_hovered.signal().for_of(|is_hovered| {
        Canvas::new(move |hx| {
            if is_hovered {
                Graphic::new(hx.rect, BUTTON_COLOR_HOVER / 255.0)
            } else {
                Graphic::new(hx.rect, BUTTON_COLOR / 255.0)
            }
        })        
    });

    let tap_area = Canvas::new(move |hx| {
        Tap::new(hx.rect, effect.clone()).with_hover_state(is_hovered.clone())
          //Graphic::new(hx.rect, BUTTON_COLOR_HOVER / 255.0))
    })
    .with_minimum_size(Size2::new(10.0, 3.0) * Env::TILE_SIZE);

    list_internal![tap_area, _bg, center(label)]
}
