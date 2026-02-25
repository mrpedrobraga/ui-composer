#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

pub fn main() {
    let use_macro = true;

    if use_macro {
        UIComposer::run_tui(Terminal(app_with_macro()));
    } else {
        UIComposer::run_tui(Terminal(app()));
    }
}

fn app() -> impl Ui {
    flex(list![
        item(
            ColorBox()
                .with_color(Rgba::red())
                .with_size(Extent2::new(30.0, 10.0))
        ),
        item(
            flex(list![
                item(
                    ColorBox()
                        .with_color(Rgba::cyan())
                        .with_size(Extent2::new(25.0, 10.0))
                ),
                item(center(
                    ColorBox()
                        .with_color(Rgba::yellow())
                        .with_size(Extent2::new(10.0, 10.0))
                ))
                .with_grow(1.0),
                item(
                    ColorBox()
                        .with_color(Rgba::magenta())
                        .with_size(Extent2::new(20.0, 10.0))
                )
                .with_grow(2.0),
            ])
            .with_vertical_flow()
        )
        .with_grow(1.0),
        item(
            ColorBox()
                .with_color(Rgba::blue())
                .with_size(Extent2::new(20.0, 10.0))
        ),
    ])
}

fn app_with_macro() -> impl Ui {
    view! {
        flex [
            item ColorBox {size:Extent2::new(30.0, 10.0), color:Rgba::cyan()} ()
            item {grow: 1.0} flex {vertical_flow} [
                item ColorBox {size:Extent2::new(25.0, 10.0), color:Rgba::red()} ()
                item {grow: 1.0} center ColorBox {size:Extent2::new(10.0, 10.0), color:Rgba::green()} ()
                item {grow: 2.0} ColorBox {size:Extent2::new(20.0, 10.0), color:Rgba::blue()} ()
            ]
            item ColorBox {size:Extent2::new(20.0, 10.0), color:Rgba::magenta()} ()
        ]
    }
}
