#![allow(non_snake_case)]
use ui_composer::{interaction::hover::Hover, prelude::*};
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
    Square(AABB::new((0, 0), (64, 64)))
}

fn Square(aabb: AABB) -> impl UIFragment {
    let hover = Hover::new(aabb);
    let state = hover.get_state();

    (Rect(aabb, (1.0, 0.0, 0.0)), hover)
}

fn Rect(aabb: AABB, color: (f32, f32, f32)) -> Primitive {
    Primitive {
        transform: [
            [aabb.size.0 as f32, 0.0, 0.0, 0.0],
            [0.0, aabb.size.1 as f32, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [aabb.top_left.0 as f32, aabb.top_left.1 as f32, 0.0, 1.0],
        ],
        color: [color.0, color.1, color.2],
    }
}
