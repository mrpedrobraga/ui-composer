#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    UIComposer::run_tui(view! {
        Terminal PanelContainer center App
    })
}

fn App() -> impl TUI {
    let lab1 = Label("I wanna code in \u{e781}.");
    let lab2 = Label("Hello, again...");

    view! {
        flex (vertical_flow) [
            item {{lab1}}
            item {{lab2}}
        ]
    }
}
