#![allow(non_snake_case)]
use ui_composer::{prelude::*, standard::ui_fragment_impls::SizedVec};
use winit::{dpi::PhysicalSize, platform::x11::WindowAttributesExtX11, window::WindowAttributes};

fn main() {
    AppBuilder::new(MyApp())
        .with_window_attributes(
            WindowAttributes::default()
                .with_title("Grid Example")
                .with_name("UI Composer", "UI Composer Grid Example")
                .with_inner_size(PhysicalSize {
                    width: 64 * 16,
                    height: 64 * 16,
                }),
        )
        .run();
}

fn MyApp() -> impl UIFragment {
    SquareGrid()
}

fn SquareGrid() -> impl UIFragment {
    const SIZE: usize = 1024;
    const SQUARE_SIZE: usize = SIZE * SIZE;
    println!("Rendering {}x{}={} quads!", SIZE, SIZE, SIZE * SIZE);

    (0..SIZE)
        .flat_map(|y| {
            let square_size = [1.0, 1.0];
            let padding = 0.0;

            (0..SIZE).map(move |x| Primitive {
                transform: aabb(
                    [
                        (x as f32 * (1.0 + padding)) * square_size[0],
                        (y as f32 * (1.0 + padding)) * square_size[1],
                    ],
                    [square_size[0], square_size[1]],
                ),
                color: [x as f32 / SIZE as f32, y as f32 / SIZE as f32, 0.0],
            })
        })
        .collect::<SizedVec<_, SQUARE_SIZE>>()
}

fn aabb(position: [f32; 2], size: [f32; 2]) -> [[f32; 4]; 4] {
    return [
        [size[0], 0.0, 0.0, 0.0],
        [0.0, size[1], 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [position[0], position[1], 0.0, 1.0],
    ];
}
