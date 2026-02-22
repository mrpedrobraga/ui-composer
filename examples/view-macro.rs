#![allow(non_snake_case)]
use lullaby_ui::{
    layout::{flex, item},
    primitives::graphic::Graphic,
};
use ui_composer::list;
use ui_composer::prelude::UIComposer;
use ui_composer_core::app::composition::layout::{ItemBox, Resizable};
use ui_composer_platform_tui::{TUI, Terminal};
use ui_composer_view_macro::view;
use vek::{Extent2, Rgba};

static SURFACE_COLOR: Rgba<f32> = Rgba::new(255.0, 253.0, 248.0, 255.0);
static SURFACE_COLOR_2: Rgba<f32> = Rgba::new(255.0, 241.0, 231.0, 255.0);

static BUTTON_COLOR: Rgba<f32> = Rgba::new(255.0, 217.0, 179.0, 255.0);
static BUTTON_COLOR_HOVER: Rgba<f32> = Rgba::new(235.0, 189.0, 143.0, 255.0);
static BUTTON_TEXT_COLOR: Rgba<f32> = Rgba::new(175.0, 90.0, 16.0, 255.0);
static TEXT_COLOR: Rgba<f32> = Rgba::new(156.0, 78.0, 10.0, 255.0);

fn main() {
    UIComposer::run_tui(view! (
        Terminal {
            flex (vertical_flow) [
                item (grow=1.0) flex [
                    item Square {{ SURFACE_COLOR / 255.0 }}
                    item (grow=2.0) Square {{ SURFACE_COLOR_2 / 255.0 }}
                    item Square {{ BUTTON_COLOR / 255.0 }}
                ]
                item Square {{ BUTTON_COLOR_HOVER / 255.0 }}
            ]
        }
    ));
}

/// A simple coloured square.
fn Square(color: Rgba<f32>) -> impl TUI {
    view! (
        ItemBox::new (minimum_size=Extent2::new(16.0, 2.0)) {{
            move |hx| view! { Graphic (color=color rect=hx.rect) }
        }}
    )
}
