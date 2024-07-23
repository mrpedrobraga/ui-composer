#![allow(non_snake_case)]
use ui_composer::{prelude::*, standard::ui_fragment_impls::SizedVec};
use winit::{dpi::PhysicalSize, platform::x11::WindowAttributesExtX11, window::WindowAttributes};

fn main() {
    AppBuilder::new(MyApp())
        .with_window_attributes(
            WindowAttributes::default()
                .with_title("Who needs fragment shaders?")
                .with_name("UI Composer Stress Test", "UI Composer Grid Example")
                .with_inner_size(PhysicalSize {
                    width: 1000,
                    height: 1000,
                }),
        )
        .run();
}

fn MyApp() -> impl UIFragment {
    SquareGrid()
}

fn SquareGrid() -> impl UIFragment {
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;
    const AREA: usize = WIDTH * HEIGHT;
    println!("Rendering {}x{}={} quads!", WIDTH, HEIGHT, AREA);

    (0..HEIGHT)
        .flat_map(|y| {
            let square_size = [1.0, 1.0];
            let padding = 0.0;

            (0..WIDTH).map(move |x| Primitive {
                transform: aabb(
                    [
                        (x as f32 * (1.0 + padding)) * square_size[0],
                        (y as f32 * (1.0 + padding)) * square_size[1],
                    ],
                    [square_size[0], square_size[1]],
                ),
                color: [x as f32 / WIDTH as f32, y as f32 / WIDTH as f32, 0.0],
            })
        })
        .collect::<SizedVec<_, AREA>>()
}

fn aabb(position: [f32; 2], size: [f32; 2]) -> [[f32; 4]; 4] {
    return [
        [size[0], 0.0, 0.0, 0.0],
        [0.0, size[1], 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [position[0], position[1], 0.0, 1.0],
    ];
}
