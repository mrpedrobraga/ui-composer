#![allow(non_snake_case)]

use ui_composer::app::composition::elements::Blueprint;
use ui_composer::prelude::UIComposer;
use ui_composer::runners::tui::Graphic;
use ui_composer::runners::winit::runner::{WinitEnvironment, WinitRunner};
use vek::{Rect, Rgba};

fn main() {
    UIComposer::run_custom::<WinitRunner<_>>(App())
}

fn App() -> impl Blueprint<WinitEnvironment, Element: Send> + Send {
    (
        Graphic {
            rect: Rect::new(0.0, 0.0, 20.0, 20.0),
            color: Rgba::new(0.0, 0.0, 1.0, 1.0),
        },
        Graphic {
            rect: Rect::new(0.0, 0.0, 20.0, 20.0),
            color: Rgba::new(0.0, 0.0, 1.0, 1.0),
        },
    )
}
