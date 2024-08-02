#![allow(non_snake_case)]
use ui_composer::{
    app::AppBuilder,
    ui::layout::{LayoutItem, Resizable},
};
use vek::Extent2;

fn main() {
    let ui = App();
    AppBuilder::new(ui).run();
}

fn App() -> impl LayoutItem {
    Resizable::new(Extent2::new(10.0, 10.0), |_| ())
}
