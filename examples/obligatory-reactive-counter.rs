#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    let counter = Mutable::new(0);

    UIComposer::run_tui(Terminal(PanelContainer(Counter(counter))))
}

fn Counter(count: Mutable<i32>) -> impl Tui {
    let txt_count = count
        .signal()
        .react(|count| Label(format!("Count: {count}")));

    view! {
        center flex [
            item Button (
                Label (("Take 1"))
                (count.clone().effect(|e| *e -= 1))
            )
            item {grow: 1.0} center ((txt_count))
            item Button (
                Label (("Add 1"))
                (count.effect(|e| *e += 1))
            )
        ]
    }
}
