#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

pub fn main() {
    UIComposer::run_tui(Terminal(app()));
}

fn app() -> impl Ui {
    let t = "The quick brown fox jumps and jumps and keeps going on and on man it really does go on forever and ever...".to_string();

    view! {
        column [
            center flex {vertical_flow} [
                item ColorBox {color: Rgba::red()} ()
                item {grow: 1.0} linewise_flow ((MonospaceText(t.clone(), Rgba::white())))
                item ColorBox {color: Rgba::blue()} ()
            ]
            center flex {vertical_flow} [
                item ColorBox {color: Rgba::yellow()} ()
                item {grow: 1.0} linewise_flow ((MonospaceText(t.clone(), Rgba::blue())))
                item ColorBox {color: Rgba::green()} ()
            ]
        ]
    }
}
