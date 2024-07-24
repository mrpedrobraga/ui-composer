#![allow(non_snake_case)]
use seq_macro::seq;
use ui_composer::{interaction::hover::HoverInteraction, prelude::*};
use winit::{dpi::LogicalSize, platform::x11::WindowAttributesExtX11, window::WindowAttributes};

fn main() {
    AppBuilder::new(MyApp())
        .with_window_attributes(
            WindowAttributes::default()
                .with_name("Simple App", "Simple App")
                .with_inner_size(LogicalSize {
                    width: 1000,
                    height: 1000,
                }),
        )
        .run();
}

fn MyApp() -> impl UIFragment {
    const WIDTH: i32 = 100;
    seq!(I in 0..1000 {
        [
            #(
                TinySquare(I - (WIDTH * (I/WIDTH)), I/WIDTH),
            )*
        ]
    })
}

fn TinySquare(x: i32, y: i32) -> impl UIFragment {
    const SIZE: i32 = 10;
    Square(AABB::new((x * SIZE, y * SIZE), (SIZE, SIZE)))
}

fn Square(aabb: AABB) -> impl UIFragment {
    let hover = HoverInteraction::new(aabb);
    let is_hovering_state = hover.get_signal();

    let square = is_hovering_state
        .map(move |is_hovering| {
            if is_hovering {
                Rect(aabb, (1.0, 0.0, 0.0))
            } else {
                Rect(aabb, (0.0, 1.0, 1.0))
            }
        })
        .into_fragment();

    (square, hover)
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
