#![allow(non_snake_case)]

use ui_composer::{app::AppBuilder, gpu::window::View, ui::layout::LayoutItem};
use vek::Extent2;

fn main() {
    let ui = App();
    AppBuilder::new(ui).run();
}

fn App() -> impl LayoutItem {
    View(Extent2::new(100.0, 100.0), ())
}
