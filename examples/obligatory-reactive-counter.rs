#![allow(unused_imports, non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    let counter = Mutable::new(0);
    TuiPlatform::run(Terminal(PanelContainer(Counter(counter))));
}

fn Counter(count: Mutable<i32>) -> impl Tui {
    view! {
        center
        Flex (vertical_flow) [
            item (grow: 1.0)
            for count of count.signal() {
                Label { format!("Count: {count}")}
                    .with_size(Size2::new(5.0, 5.0))
                    .centered()
            }

            item
            Row [
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
