#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

pub fn main() {
    UIComposer::run_tui(Terminal(app()));
}

fn app() -> impl Ui {
    let t = "The quick brown fox jumps and jumps and keeps going on and on man it really does go on forever and ever...".to_string();
    let size = Size2::new(4.0, 4.0);

    view! {
        column [
            flex {} [
                item ColorBox {color: Srgba::new(1.0, 0.0, 0.0, 1.0), size: size} ()
                item {grow: 1.0} linewise_flow ((MonospaceText(t.clone(), Srgba::new(1.0, 1.0, 0.0, 1.0))))
                item ColorBox {color: Srgba::new(0.0, 0.0, 1.0, 1.0), size: size} ()
            ]
            center flex {} [
                item ColorBox {color: Srgba::new(1.0, 1.0, 0.0, 1.0), size: size} ()
                item {grow: 1.0} linewise_flow ((MonospaceText(t.clone(), Srgba::new(1.0, 0.0, 1.0, 1.0))))
                item ColorBox {color: Srgba::new(0.0, 1.0, 0.0, 1.0), size: size} ()
            ]
        ]
    }
}
