#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    UIComposer::run_tui(Terminal(app()))
}

fn app() -> impl TUI {
    inline_flow(list![
        Inline(Square(Rgba::red())),
        Inline(Square(Rgba::green())),
        MonospaceText("Hello, there!".to_string()),
        Inline(Square(Rgba::magenta())),
        Inline(Square(Rgba::blue())),
        Inline(Square(Rgba::red())),
        Inline(Square(Rgba::green())),
        Inline(Square(Rgba::magenta())),
        Inline(Square(Rgba::blue())),
        Inline(Square(Rgba::red())),
        Inline(Square(Rgba::green())),
        Inline(Square(Rgba::magenta())),
        Inline(Square(Rgba::blue()))
    ])
}

fn Square(color: Rgba<f32>) -> impl TUI {
    ItemBox::new(move |hx| Graphic {
        rect: hx.rect,
        color,
    })
    .with_minimum_size(Extent2::new(16.0, 4.0))
}
