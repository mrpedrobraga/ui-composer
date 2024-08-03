#![allow(non_snake_case)]

use futures_signals::signal::Mutable;
use ui_composer::{
    app::AppBuilder,
    gpu::window::{Window, WindowAttributes, WindowNode},
    ui::{
        graphics::Quad,
        layout::{LayoutItem, Resizable},
        node::UINode,
    },
};
use vek::{Extent2, Rect, Rgb};

fn main() {
    let ui = App();
    AppBuilder::new(ui).run();
}

fn App() -> WindowNode<impl UINode> {
    let window_attributes = WindowAttributes {
        title: Mutable::new("My Window".into()),
    };

    Window(window_attributes, Empty())
}

fn Empty() -> impl LayoutItem {
    Resizable::new(Extent2::new(100.0, 100.0), |rect| {
        Quad::rect(Rect::new(0.0, 0.0, 1.0, 1.0), Rgb::red())
    })
}
