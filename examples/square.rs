#![allow(non_snake_case)]
use signal_vec::MutableVec;
use ui_composer::{gpu::dynamic::VecItem, prelude::*};

pub fn main() {
    App::run(Window(Squares()));
}

#[allow(non_snake_case)]
pub fn Squares() -> impl LayoutItem {
    let mvec = MutableVec::new();
    mvec.lock_mut()
        .push(Quad::new(Rect::new(0.0, 0.0, 64.0, 32.0), Rgb::yellow()));

    Resizable::new(move |hx| VecItem::new(hx.rect, mvec.clone()))
}
