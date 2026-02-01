#![allow(non_snake_case)]
use ui_composer::app::backend::Runner;
use ui_composer::geometry::layout::{ItemBox, Resizable};
use ui_composer::runners::tui::nodes::{TUI, Terminal};
use ui_composer::runners::tui::{Graphic, TUIRunner};
use vek::{Extent2, Rect, Rgba};
use ui_composer::standard::{Center, Row};

fn main() {
    TUIRunner::run(Terminal(App()))
}

fn App() -> impl TUI {
    Center(
        Row(Square(Rgba::green()), Square(Rgba::red())),
    )
}

fn Square(color: Rgba<f32>) -> impl TUI {
    ItemBox::new(move |hx| Graphic {
        rect: hx.rect,
        color,
    }).with_minimum_size(Extent2::new(32.0, 16.0))
}