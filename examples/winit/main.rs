#![allow(non_snake_case)]

use vek::{Rect, Rgba};
use ui_composer::prelude::UIComposer;
use ui_composer::runners::tui::Graphic;
use ui_composer::runners::winit::runner::WinitRunner;

fn main() {
    #[allow(clippy::unit_arg)]
    UIComposer::run_custom::<WinitRunner<_>>(App())
}

fn App() -> (Graphic, Graphic) {
    (
        Graphic {
            rect: Rect::new(0.0, 0.0, 20.0, 20.0),
            color: Rgba::new(0.0, 0.0, 1.0, 1.0),
        },
        Graphic {
            rect: Rect::new(0.0, 0.0, 20.0, 20.0),
            color: Rgba::new(0.0, 0.0, 1.0, 1.0),
        }
    )
}
