#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    UIComposer::run_tui(Terminal(app()))
}

fn app() -> impl TUI {
    inline_flow(list![
        Inline(Square(2.0, Rgba::red())),
        Inline(Square(7.0, Rgba::green())),
        MonospaceText(
            "This is an amazing opportunity to show how cool layouting is!"
                .to_string()
        ),
        Inline(Square(9.0, Rgba::magenta())),
        Inline(Square(4.0, Rgba::blue())),
        Inline(Square(5.0, Rgba::red())),
        Inline(Square(7.0, Rgba::green())),
        Inline(Square(2.0, Rgba::magenta())),
        Inline(Square(1.0, Rgba::blue())),
        MonospaceText("What the hell?".to_string()),
        Inline(Square(1.0, Rgba::red())),
        Inline(Square(3.0, Rgba::green())),
        Inline(Square(5.0, Rgba::magenta())),
        Inline(Square(4.0, Rgba::blue()))
    ])
}

fn Square(w: f32, color: Rgba<f32>) -> impl TUI {
    ItemBox::new(move |hx| Graphic {
        rect: hx.rect,
        color,
    })
    .with_minimum_size(Extent2::new(w * 3.0, 4.0))
}
