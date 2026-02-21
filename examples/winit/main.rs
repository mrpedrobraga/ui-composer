#![allow(non_snake_case)]

use ui_composer::prelude::UIComposer;
use ui_composer_core::app::composition::elements::Blueprint;
use ui_composer_platform_winit::runner::WinitEnvironment;

fn main() {
    UIComposer::run_winit(App())
}

fn App() -> impl Blueprint<WinitEnvironment, Element: Send> + Send {
    // (
    //     Graphic {
    //         rect: Rect::new(0.0, 0.0, 20.0, 20.0),
    //         color: Rgba::new(0.0, 0.0, 1.0, 1.0),
    //     },
    //     Graphic {
    //         rect: Rect::new(0.0, 0.0, 20.0, 20.0),
    //         color: Rgba::new(0.0, 0.0, 1.0, 1.0),
    //     },
    // )
}
