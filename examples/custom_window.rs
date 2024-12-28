#![allow(unused, non_snake_case)]

use ui_composer::items;
use ui_composer::prelude::*;

fn main() {
    let window = Window(App())
        .with_title("Custom window!")
        .with_decorations(false);

    UIComposer::run(window);
}

fn App() -> impl LayoutItem {
    Center(Drag())
}

fn Drag() -> impl LayoutItem {
    let s1 = Mutable::new(false);
    let s2 = Mutable::new(false);

    ResizableItem::new(move |parent| {
        let window_drag = WindowDrag::new(parent.rect, s1.clone(), s2.clone());

        items! {
            window_drag,
            parent.rect.with_color(Rgb::new(1.0, 1.0, 1.0))
        }
    })
    .with_minimum_size(Extent2::new(100.0, 100.0))
}
