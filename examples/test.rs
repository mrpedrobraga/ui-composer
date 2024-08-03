#![allow(non_snake_case)]
use ui_composer::prelude::*;

fn main() {
    App::run(Main());
}

fn Main() -> impl Node {
    Window(Square())
}

fn Square() -> impl LayoutItem {
    Resizable::new(|hx| Quad::new(hx.rect, Rgb::red()))
}
