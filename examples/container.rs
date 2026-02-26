#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

pub fn main() {
    let use_macro = false;

    if use_macro {
        UIComposer::run_tui(Terminal(app_with_macro()));
    } else {
        UIComposer::run_tui(Terminal(app()));
    }
}

fn app() -> impl Ui {
    let t = "The quick brown fox jumps and jumps and keeps going on and on man it really does go on forever and ever...".to_string();

    view! {
        center flex [
            item ColorBox {color: Rgba::red()} ()
            item {grow: 1.0} linewise_flow MonospaceText ((t) (Rgba::white()))
            item ColorBox {color: Rgba::blue()} ()
        ]
    }
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
