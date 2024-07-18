#![allow(non_snake_case)]
use ui_composer::prelude::*;

fn main() {
    AppBuilder::new(MyApp()).run();
}

fn MyApp() -> impl UIFragment {
    SquareGrid()
}

fn SquareGrid() -> impl UIFragment {
    [
        Rect((0.0, 0.0), (16.0, 16.0), (1.0, 0.0, 1.0)),
        Rect((100.0, 100.0), (16.0, 16.0), (1.0, 0.0, 1.0)),
    ]
}

fn Rect(top_left: (f32, f32), size: (f32, f32), color: (f32, f32, f32)) -> Primitive {
    Primitive {
        transform: [
            [size.0, 0.0, 0.0, 0.0],
            [0.0, size.1, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [top_left.0, top_left.1, 0.0, 1.0],
        ],
        color: [color.0, color.1, color.2],
    }
}
