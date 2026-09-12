#![allow(non_snake_case)]

use ui_composer_basic_ui::primitives::graphic::Graphic;
use ui_composer_core::app::composition::{layout::{ItemBox}};
use ui_composer_math::{
    glamour::{Point2, Rect, Size2},
    palette::Srgba,
};
use {
    ui_composer::prelude::UIComposer,
    ui_composer_platform_winit::window::Window,
};

fn main() {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .without_time()
    //     .init();

    UIComposer::run_winit(Window(App()))
}

fn App() -> ItemBox<impl FnMut(ui_composer_core::app::composition::layout::hints::ParentHints) -> (Graphic, Graphic), (Graphic, Graphic)> {
    ItemBox::new(|hints| {
        App2()
    })
}

fn App2() -> (Graphic, Graphic)  {
    (
        Graphic {
            rect: Rect::new(Point2::new(0.0, 0.0), Size2::new(20.0, 20.0)),
            color: Srgba::new(0.0, 0.0, 1.0, 1.0),
        },
        Graphic {
            rect: Rect::new(Point2::new(10.0, 10.0), Size2::new(20.0, 20.0)),
            color: Srgba::new(1.0, 1.0, 0.0, 1.0),
        },
    )
}
