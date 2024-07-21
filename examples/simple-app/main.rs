#![allow(non_snake_case)]
use futures_signals::signal::SignalExt;
use ui_composer::{interaction::tap::TapInteraction, prelude::*};
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
    let tap = TapInteraction::new(aabb);

    let signal = tap.get_signal();
    std::thread::spawn(move || {
        pollster::block_on(signal.for_each(|click| async move {
            if click.is_some() {
                println!("Clicked!")
            };
        }))
    });

    (Rect(aabb, (1.0, 0.0, 0.0)), tap)
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
