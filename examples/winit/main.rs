#![allow(non_snake_case, unused)]

use ::ui_composer_core::app::composition::CompatibleWith;
use ::ui_composer_state::effect::animation::assign;
use ui_composer::prelude::*;
use ui_composer_basic_ui::{
    layout::{flex, item, row},
    primitives::graphic::Graphic,
};
use ui_composer_math::{glamour::Rect, palette::rgb::Rgba};
use ui_composer_platform_winit::window::Window;
use ui_composer_state::effect::animation::{futures_time::time::Duration, Animation};

fn main() {
    //UIComposer::run_winit(Window(App()));
    UIComposer::run_tui(Terminal(App()))
}

fn App2() -> impl Tui {
    let point_state = Mutable::new(Point2::new(0.0, 0.0));
    let point_signal = point_state.signal();

    let animation_process = assign(Point2::new(0.0, 0.0))
        .then_lerp_to(Point2::new(200.0, 0.0), Duration::from_secs(1))
        .then_lerp_to(Point2::new(200.0, 200.0), Duration::from_secs(1))
        .then_lerp_to(Point2::new(0.0, 200.0), Duration::from_secs(1))
        .animate_value(point_state)
        .into_ui_process();

    let other_future = async {
        ::ui_composer_state::effect::animation::futures_time::task::sleep(Duration::from_secs(1))
            .await;
        Srgba::new(0.8, 0.2, 0.0, 1.0)
    };

    view! {
        (animation_process)

        row [
            // Awaiting a future
            for color of other_future {
                ColorBox (
                    ( Rect::new(Point2::new(100.0, 100.0), Size2::new(50.0, 50.0)) )
                    ( color )
                )
            } else {
                ColorBox (
                    ( Rect::new(Point2::new(100.0, 100.0), Size2::new(50.0, 50.0)) )
                    ( Srgba::new(0.5, 0.5, 0.5, 1.0) )
                )
            }

            // Reacting to a signal
            for point of point_signal {
                ColorBox (
                    ( Rect::new(point, Size2::new(100.0, 100.0)) )
                    ( Srgba::new(0.8, 0.7, 0.0, 1.0) )
                )
            }

            ColorBox (
                ( Rect::new(Point2::new(200.0, 200.0), Size2::new(50.0, 50.0)) )
                ( Srgba::new(0.2, 0.7, 0.0, 1.0) )
            )
        ]
    }
}

fn ColorBox<Env: Environment>(rect: Rect, color: Rgba) -> impl CompatibleWith<Env>
where
    ui_composer_basic_ui::primitives::graphic::Graphic: ui_composer_core::prelude::Blueprint<Env>,
{
    Canvas::new(move |_| Graphic { rect, color })
}

fn App() -> impl Tui {
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

fn LaserSquares(rect: Rect) -> Vec<Graphic> {
    let colors = [
        Srgba::new(1.0, 1.0, 0.0, 1.0),
        Srgba::new(0.0, 1.0, 0.0, 1.0),
        Srgba::new(0.0, 1.0, 1.0, 1.0),
        Srgba::new(0.0, 0.0, 1.0, 1.0),
        Srgba::new(0.0, 0.0, 0.0, 1.0),
        Srgba::new(1.0, 0.0, 0.0, 1.0),
    ];

    let size = Size2::<f32>::new(4.0, 2.0);
    let step = 2.0;

    let mut velocity = Vector2::<f32>::new(1.0, 1.0);
    let mut position = Point2::<f32>::new(0.0, 0.0);

    (0..99)
        .map(|i| {
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

            position += velocity * step;

            g
        })
        .chain(std::iter::once(Graphic {
            rect,
            color: Srgba::new(0.2, 0.3, 0.9, 1.0),
        }))
        .rev()
        .collect()
}
