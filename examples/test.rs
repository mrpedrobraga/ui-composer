#![allow(non_snake_case)]
use ui_composer::prelude::*;

pub fn main() {
    App::run(MyApp());
}

pub fn MyApp() -> impl Node {
    Window(Empty())
}
