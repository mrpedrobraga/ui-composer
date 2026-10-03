#![allow(non_snake_case)]
use {
    ::ui_composer_platform_winit::window::Window, lullaby_ui::prelude::*, ui_composer::prelude::*,
};

fn main() {
    let counter = Mutable::new(0);

    //TuiPlatform::run(Terminal(PanelContainer(Counter(counter))))
    DesktopPlatform::run(Window(PanelContainer(Counter(counter))));
}

fn Counter(count: Mutable<i32>) -> impl DesktopUi {
    // view! {
    //     Button ( Label(("Test Button")) (||{}) )
    // }

    view! {
        center flex [
            item Button (
                Label ("Take 1")
                (count.clone().effect(|e| *e -= 1))
            )
            item {grow: 1.0}
            for count of count.signal() {
                center Label (( format!("Count: {count}") ))
            }
            item Button (
                Label ("Add 1")
                (count.effect(|e| *e += 1))
            )
        ]
    }
}
