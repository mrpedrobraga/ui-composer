#![allow(non_snake_case)]
use ui_composer::prelude::*;

pub fn main() {
    App::run(
        Window(Resizable::new(|hx| {
            //Quad::new(Rect::new(16.0, 16.0, 32.0, 32.0), Rgb::blue())
        }))
        .with_title("Hello window!"),
    );
}
