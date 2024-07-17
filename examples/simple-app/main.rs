#![allow(non_snake_case)]
use ui_composer::prelude::*;

fn main() {
    AppBuilder::new(MyApp()).run();
}

fn MyApp() -> impl UIFragment {
    Button("Click me")
}

fn rect(position: [f32; 2]) -> [[f32; 4]; 4] {
    return [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [position[0], position[1], 0.0, 1.0],
    ];
}

fn Button(label_text: &str) -> impl UIFragment {
    (
        Primitive {
            color: [0.5, 0.5, 1.0],
            transform: rect([0.5, 0.5]),
        },
        Primitive {
            color: [1.0, 0.4, 0.0],
            transform: rect([0.2, 0.8]),
        },
    )
}
