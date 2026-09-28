#![allow(non_snake_case)]

use ui_composer::prelude::*;
use ui_composer_basic_ui::primitives::graphic::Graphic;
use ui_composer_core::app::composition::effects::future::Await;
use ui_composer_math::{glamour::Rect, palette::rgb::Rgba};
use ui_composer_platform_winit::window::Window;
use ui_composer_state::effect::animation::futures_time::{task::sleep, time::Duration};
fn main() {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .without_time()
    //     .init();

    let window = Window(App());
    UIComposer::run_winit(window);
}

fn App() -> impl WinitUi {
    //Canvas::new(|hx| LaserSquares(hx.rect))

    let future = async {
        println!("Eeping");
        sleep(Duration::from_secs(1)).await;
        Point2::new(200.0, 100.0)
    };

    Await::new(future, |origin| {
        ColorBox(
            Rect::new(origin, Size2::new(100.0, 100.0)),
            Srgba::new(1.0, 1.0, 0.0, 1.0),
        )
    })
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
