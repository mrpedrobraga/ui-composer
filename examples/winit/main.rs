#![allow(non_snake_case)]

use futures::executor::block_on;
use ui_composer::prelude::*;
use ui_composer_basic_ui::{
    layout::{flex, item, row},
    primitives::graphic::Graphic,
};
use ui_composer_math::{glamour::Rect, palette::rgb::Rgba};
use ui_composer_platform_winit::window::Window;
use ui_composer_state::effect::animation::futures_time::{task::sleep, time::Duration};

fn main() {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .without_time()
    //     .init();

    let window = Window(App2());
    UIComposer::run_winit(window);
}

#[allow(unused)]
fn App() -> impl WinitUi {
    let c_a = Canvas::new(|hx| LaserSquares(hx.rect)).with_minimum_size(Size2 {
        width: 400.0,
        height: 400.0,
    });
    let c_b = Canvas::new(|hx| LaserSquares(hx.rect)).with_minimum_size(Size2 {
        width: 400.0,
        height: 400.0,
    });

    view! {
        flex {vertical_flow} [
            item ((c_a))
            item {grow: 1.0} ((c_b))
        ]
    }
}

#[allow(unused)]
fn App2() -> impl WinitUi {
    let point_state = Mutable::new(Point2::new(0.0, 0.0));
    let point_signal = point_state.signal();

    std::thread::spawn(move || {
        block_on(async move {
            sleep(Duration::from_secs(1)).await;
            point_state.set(Point2::new(200.0, 0.0));
            sleep(Duration::from_secs(1)).await;
            point_state.set(Point2::new(200.0, 200.0));
            sleep(Duration::from_secs(1)).await;
            point_state.set(Point2::new(0.0, 200.0));
        })
    });

    view! {
        row [
            ColorBox (
                (( Rect::new(Point2::ZERO, Size2::new(50.0, 50.0)) ))
                (( Srgba::new(0.8, 0.2, 0.0, 1.0) ))
            )
            // Fine-grained reactivity with functors*!
            for point of point_signal {
                ColorBox (
                    (( Rect::new(point, Size2::new(100.0, 100.0)) ))
                    (( Srgba::new(0.8, 0.7, 0.0, 1.0) ))
                )
            }
            ColorBox (
                (( Rect::new(Point2::new(200.0, 200.0), Size2::new(50.0, 50.0)) ))
                (( Srgba::new(0.2, 0.7, 0.0, 1.0) ))
            )
        ]
    }
}

#[allow(non_snake_case, unused)]
fn ColorBox(rect: Rect, color: Rgba) -> impl WinitUi {
    Canvas::new(move |_| Graphic { rect, color })
}

#[allow(non_snake_case, unused)]
fn LaserSquares(rect: Rect) -> Vec<Graphic> {
    let colors = [
        Srgba::new(1.0, 1.0, 0.0, 1.0),
        Srgba::new(0.0, 1.0, 0.0, 1.0),
        Srgba::new(0.0, 1.0, 1.0, 1.0),
        Srgba::new(0.0, 0.0, 1.0, 1.0),
        Srgba::new(0.0, 0.0, 0.0, 1.0),
        Srgba::new(1.0, 0.0, 0.0, 1.0),
    ];

    let mut velocity = Vector2::<f32>::new(1.0, 1.0);
    let mut position = Point2::<f32>::new(0.0, 0.0);

    (0..99)
        .map(|i| {
            let size = Size2::<f32>::new(20.0, 20.0);

            if position.x <= 0.0 {
                velocity.x = 1.0
            };
            if position.y <= 0.0 {
                velocity.y = 1.0
            };
            if position.x >= rect.size.width - size.width {
                velocity.x = -1.0
            };
            if position.y >= rect.size.height - size.height {
                velocity.y = -1.0
            };

            let g = Graphic {
                rect: Rect {
                    origin: rect.origin + position,
                    size,
                },
                color: colors[i % colors.len()],
            };

            position += velocity * 10.0;

            g
        })
        .chain(std::iter::once(Graphic {
            rect,
            color: Srgba::new(0.2, 0.3, 0.9, 1.0),
        }))
        .rev()
        .collect()
}
