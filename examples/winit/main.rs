#![allow(non_snake_case)]

use ui_composer::prelude::*;
use ui_composer_basic_ui::primitives::graphic::Graphic;
use ui_composer_core::app::composition::layout::item_box;
use ui_composer_math::glamour::Rect;
use ui_composer_platform_winit::window::Window;
use ui_composer_platform_winit::WinitUi;
use ui_composer_view_macro::view;

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

fn AppContent(rect: Rect) -> (Graphic, Graphic) {
    let thin_rect = Rect::new(Point2::new(0.0, 128.0), Size2::new(100.0, 20.0));

    view! {
        Graphic { rect: rect, color: Srgba::new(0.0, 0.0, 1.0, 1.0) },
        Graphic { rect: thin_rect, color: Srgba::new(1.0, 1.0, 0.2, 1.0) },
    }
}
