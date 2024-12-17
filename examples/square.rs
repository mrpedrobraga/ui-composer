#![allow(non_snake_case)]
use signal_vec::MutableVec;
use ui_composer::{gpu::dynamic::VecItem, prelude::*};

pub fn main() {
    App::run(Window(Squares()));
}

#[allow(non_snake_case)]
pub fn Squares() -> impl LayoutItem {
    let squares: MutableVec<Quad> = MutableVec::new();

    Resizable::new(|hx| Quad::new(hx.rect.expand_radius(-10.0), Rgb::new(0.5, 0.5, 0.5)))
}
