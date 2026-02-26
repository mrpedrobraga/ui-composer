#![allow(non_snake_case)]

use ui_composer_core::app::composition::elements::Blueprint;
use ui_composer_platform_winit::runner::WinitEnvironment;
use {
    ui_composer::prelude::UIComposer,
    ui_composer_platform_winit::window::window,
};

fn main() {
    UIComposer::run_winit(window(App()))
}

fn App() -> impl Blueprint<WinitEnvironment, Element: Send> + Send {
    // (
    //     Graphic {
    //         rect: Rect::new(0.0, 0.0, 20.0, 20.0),
    //         color: Srgba::new(0.0, 0.0, 1.0, 1.0),
    //     },
    //     Graphic {
    //         rect: Rect::new(0.0, 0.0, 20.0, 20.0),
    //         color: Srgba::new(0.0, 0.0, 1.0, 1.0),
    //     },
    // )
}
