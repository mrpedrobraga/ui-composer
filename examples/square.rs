#![allow(non_snake_case)]
use ui_composer::{prelude::*, tuple};

pub fn main() {
    App::run(Window(Resizable::new(|hx| {
        tuple!(
            Quad::new(hx.rect.expand_radius(-10.0), Rgb::blue()),
            Quad::new(Rect::new(8.0, 8.0, 16.0, 16.0), Rgb::red())
        )
    })));
}
