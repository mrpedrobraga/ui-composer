#![allow(non_snake_case)]

use ui_composer::prelude::*;
use ui_composer_basic_ui::primitives::graphic::Graphic;
use ui_composer_core::app::composition::layout::item_box;
use ui_composer_math::glamour::Rect;
use ui_composer_platform_winit::window::Window;
use ui_composer_platform_winit::WinitUi;

fn main() {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .without_time()
    //     .init();

    UIComposer::run_winit(Window(App()))
}

fn App() -> impl WinitUi {
    item_box(|hints| AppContent(hints.rect))
}

#[allow(non_snake_case)]
fn AppContent(rect: Rect) -> Vec<Graphic> {
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

    (0..100)
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
        .collect()
}
