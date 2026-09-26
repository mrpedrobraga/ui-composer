#![allow(non_snake_case)]

use std::time::Duration;

use futures::FutureExt;
use ui_composer::prelude::*;
use ui_composer_basic_ui::primitives::graphic::Graphic;
use ui_composer_core::app::composition::effects::signal::IntoBlueprint;
use ui_composer_core::app::composition::layout::{item_box, ItemBox2};
use ui_composer_math::glamour::Rect;
use ui_composer_platform_winit::window::Window;
use ui_composer_platform_winit::WinitUi;
use ui_composer_state::effect::animation::futures_time;
use ui_composer_state::effect::animation::futures_time::future::FutureExt as _;

fn main() {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .without_time()
    //     .init();

    //let app = App2();
    let app = item_box(|hints| LaserSquares(hints.rect));
    let window = Window(DelayedBox2());

    Mutable::new(0).signal().to_future();

    UIComposer::run_winit(window);
}

fn DelayedBox() -> impl WinitUi {
    let square: Mutable<Option<Graphic>> = Mutable::new(None);
    let square2 = square.clone();

    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(3000));
        square2.set(Some(Graphic {
            rect: Rect {
                origin: Point2 { x: 0.0, y: 0.0 },
                size: Size2 {
                    width: 100.0,
                    height: 100.0,
                },
            },
            color: Srgba::new(0.2, 0.3, 0.9, 1.0),
        }));
    });

    ItemBox2::new((square,), |(square,), _hints| {
        square.signal().into_blueprint()
    })
}

#[allow(unused)]
fn DelayedBox2() -> impl WinitUi {
    //

    let fut = async move {
        Graphic {
            rect: Rect {
                origin: Point2 { x: 0.0, y: 0.0 },
                size: Size2 {
                    width: 100.0,
                    height: 100.0,
                },
            },
            color: Srgba::new(0.2, 0.3, 0.9, 1.0),
        }
    }
    .delay(futures_time::time::Duration::from_millis(5000));

    let futs = fut.shared().into_blueprint();
    item_box(move |hints| {
        (
            Graphic {
                rect: hints.rect.inflate(Size2::new(-10.0, -10.0)),
                color: Srgba::new(0.5, 0.2, 0.7, 1.0),
            },
            futs.clone(),
        )
    })
}

#[allow(non_snake_case)]
#[allow(unused)]
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
            position += velocity * 10.0;
            if position.x < 0.0 {
                velocity.x = 1.0
            };
            if position.y < 0.0 {
                velocity.y = 1.0
            };
            if position.x > rect.size.width {
                velocity.x = -1.0
            };
            if position.y > rect.size.height {
                velocity.y = -1.0
            };
            let size = Size2::<f32>::new(20.0, 20.0);

            Graphic {
                rect: Rect {
                    origin: position,
                    size,
                },
                color: colors[i % colors.len()],
            }
        })
        .chain(std::iter::once(Graphic {
            rect,
            color: Srgba::new(0.2, 0.3, 0.9, 1.0),
        }))
        .rev()
        .collect()
}
