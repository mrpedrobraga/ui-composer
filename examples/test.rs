#![allow(non_snake_case)]
use ui_composer::{components::Center, prelude::*};

pub fn main() {
    App::run(MyApp());
}

pub fn MyApp() -> impl Node {
    Window(Center(
        Resizable::new(|hx| Quad::new(hx.rect, Rgb::cyan()))
            .with_minimum_size(Extent2::new(100.0, 100.0)),
    ))
}
