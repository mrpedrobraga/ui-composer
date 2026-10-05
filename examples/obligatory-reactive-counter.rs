#![allow(unused_imports)]
#![allow(non_snake_case)]
use {
    ::ui_composer_platform_winit::prelude::*, lullaby_ui::prelude::*, ui_composer::prelude::*,
};

fn main() {
    let counter = Mutable::new(0);

    //TuiPlatform::run(Terminal(PanelContainer(Counter(counter))));
    DesktopPlatform::run(Window(PanelContainer(Counter(counter))));
}

fn Counter(count: Mutable<i32>) -> impl DesktopUi {
    view! {
        center flex (vertical_flow) [
            item (grow: 1.0)
                for count of count.signal() {
                    with_size (size: Size2::new(5.0, 5.0)) center Label format!("Count: {count}")
                }
            item row [
                Button {
                    Label "Take 1"
                    { count.clone().effect(|e| *e -= 1) }
                }
                Button {
                    Label "Add 1"
                    { count.effect(|e| *e += 1) }
                }
            ]
        ]
    }
}
