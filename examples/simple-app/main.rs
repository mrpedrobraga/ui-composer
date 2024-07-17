#![allow(non_snake_case)]
use ui_composer::prelude::*;

fn main() {
    AppBuilder::new(MyApp()).run();
}

fn MyApp() -> impl UIFragment {
    Button("Click me")
}

fn Button(label_text: &str) -> impl UIFragment {
    (
        Primitive {
            color: [1.0, 1.0, 1.0],
        },
        Primitive {
            color: [0.5, 0.5, 1.0],
        },
    )
}
