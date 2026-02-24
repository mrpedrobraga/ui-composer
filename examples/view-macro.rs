#![allow(non_snake_case)]
use lullaby_ui::prelude::*;
use ui_composer::prelude::*;

static SURFACE_COLOR: Rgba<f32> = Rgba::new(255.0, 253.0, 248.0, 255.0);
static SURFACE_COLOR_2: Rgba<f32> = Rgba::new(255.0, 241.0, 231.0, 255.0);

static BUTTON_COLOR: Rgba<f32> = Rgba::new(255.0, 217.0, 179.0, 255.0);
static BUTTON_COLOR_HOVER: Rgba<f32> = Rgba::new(235.0, 189.0, 143.0, 255.0);

fn main() {
    let size = Extent2::new(4.0, 4.0);

    UIComposer::run_tui(view! (
        Terminal (
            flex {vertical_flow} [
                item {grow: 1.0} flex [
                    item ColorBox {size:size, color:SURFACE_COLOR/255.0}()
                    item {grow: 2.0} ColorBox {size:size, color:SURFACE_COLOR_2/255.0}()
                    item ColorBox {size:size, color:BUTTON_COLOR/255.0}()
                ]
                item ColorBox {size:size, color:BUTTON_COLOR_HOVER/255.0}()
            ]
        )
    ));
}
