#![allow(non_snake_case)]
use ui_composer::app::composition::layout::{ItemBox, Resizable};
use ui_composer::list;
use ui_composer::prelude::{UIComposer, flex};
use ui_composer::runners::tui::nodes::Terminal;
use ui_composer::runners::tui::runner::TUIRunner;
use ui_composer::runners::tui::{Graphic, TUI};
use ui_composer::standard::item;
use vek::{Extent2, Rgba};

fn main() {
    UIComposer::run_custom::<TUIRunner<_>>(Terminal(
        flex(list!(
            item(Square(Rgba::red())).with_grow(0.0),
            item(Square(Rgba::green())).with_grow(1.0),
            item(Square(Rgba::blue())).with_grow(0.0),
        ))
        .with_vertical_flow(),
    ))
}

fn Square(color: Rgba<f32>) -> impl TUI {
    ItemBox::new(move |hx| Graphic {
        rect: hx.rect,
        color,
    })
    .with_minimum_size(Extent2::new(16.0, 4.0))
}
