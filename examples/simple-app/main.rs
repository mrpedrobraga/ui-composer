#![allow(non_snake_case)]
use ui_composer::prelude::*;
use winit::{dpi::LogicalSize, platform::x11::WindowAttributesExtX11, window::WindowAttributes};

fn main() {
    AppBuilder::new(MyApp())
        .with_window_attributes(
            WindowAttributes::default()
                .with_name("Simple App", "Simple App")
                .with_inner_size(LogicalSize {
                    width: 128,
                    height: 128,
                }),
        )
        .run();
}

fn MyApp() -> impl UIFragment {
    SquareGrid()
}

fn SquareGrid() -> impl UIFragment {
    [Point((32.0, 32.0)), Point((128.0 - 32.0, 32.0))]
}

fn Point(position: (f32, f32)) -> Primitive {
    Rect(
        (position.0 - 8.0, position.1 - 8.0),
        (16.0, 16.0),
        (0.0, 0.0, 0.0),
    )
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
