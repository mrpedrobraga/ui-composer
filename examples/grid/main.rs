#![allow(non_snake_case)]
use ui_composer::prelude::*;

fn main() {
    AppBuilder::new(MyApp()).run();
}

fn MyApp() -> impl UIFragment {
    SquareGrid()
}

fn SquareGrid() -> impl UIFragment {
    let size = 4 * 4;
    (0..size)
        .flat_map(|y| {
            let square_size = [64.0, 64.0];
            let padding = 0.1;

            (0..size).map(move |x| Primitive {
                transform: aabb(
                    [
                        (x as f32 * (1.0 + padding)) * square_size[0],
                        (y as f32 * (1.0 + padding)) * square_size[1],
                    ],
                    [square_size[0], square_size[1]],
                ),
                color: [x as f32 / size as f32, y as f32 / size as f32, 0.0],
            })
        })
        .collect::<Vec<_>>()
}

fn aabb(position: [f32; 2], size: [f32; 2]) -> [[f32; 4]; 4] {
    return [
        [size[0], 0.0, 0.0, 0.0],
        [0.0, size[1], 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [position[0], position[1], 0.0, 1.0],
    ];
}
