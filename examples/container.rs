#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

pub fn main() {
    UIComposer::run_tui(Terminal(app()));
}

fn app() -> impl Ui {
    flex(list![
        item(
            ColorBox()
                .with_color(Rgba::red())
                .with_size(Extent2::new(20.0, 20.0))
        ),
        item(
            ColorBox()
                .with_color(Rgba::green())
                .with_size(Extent2::new(20.0, 20.0))
        )
        .with_grow(1.0),
        item(
            ColorBox()
                .with_color(Rgba::blue())
                .with_size(Extent2::new(20.0, 20.0))
        ),
    ])
}
