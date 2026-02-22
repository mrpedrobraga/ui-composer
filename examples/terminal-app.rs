#![allow(non_snake_case)]

use lullaby_ui::{
    layout::{flex, item},
    primitives::graphic::Graphic,
};
use ui_composer::{list, prelude::UIComposer};
use ui_composer_core::app::composition::layout::{ItemBox, Resizable as _};
use ui_composer_platform_tui::{TUI, Terminal, runner::TUIRunner};
use vek::{Extent2, Rgba};

fn main() {
    let app = flex(list![
        item(Square(Rgba::new(255, 235, 252, 255).as_() / 255.0))
            .with_grow(2.0),
        item(Square(Rgba::new(245, 219, 241, 255).as_() / 255.0))
            .with_grow(1.0),
        item(Square(Rgba::new(134, 70, 139, 255).as_() / 255.0)).with_grow(0.0),
    ]);
    UIComposer::run_custom::<TUIRunner<_>>(Terminal(app))
}

fn Square(color: Rgba<f32>) -> impl TUI {
    ItemBox::new(move |hx| Graphic {
        rect: hx.rect,
        color,
    })
    .with_minimum_size(Extent2::new(16.0, 4.0))
}
