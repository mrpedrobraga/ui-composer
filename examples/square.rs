#![allow(non_snake_case)]
use ui_composer::{prelude::*, tuple};

pub fn main() {
    App::run(tuple!(
        Window(Empty()),
        Window(Empty()),
        Window(Resizable::new(|hx| Quad::new(hx.rect, Rgb::green())))
    ));
}
