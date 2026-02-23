#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    let app = flex(list![
        item(Square(Rgba::new(255, 235, 252, 255).as_() / 255.0))
            .with_grow(2.0),
        item(Square(Rgba::new(245, 219, 241, 255).as_() / 255.0))
            .with_grow(1.0),
        item(Square(Rgba::new(134, 70, 139, 255).as_() / 255.0)).with_grow(0.0),
    ]);
    UIComposer::run_tui(Terminal(app))
}

fn Square(color: Rgba<f32>) -> impl TUI {
    ItemBox::new(move |hx| Graphic {
        rect: hx.rect,
        color,
    })
    .with_minimum_size(Extent2::new(16.0, 4.0))
}
