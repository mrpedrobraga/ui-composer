#![allow(non_snake_case)]
use ui_composer::prelude::*;

fn main() {
    let ui = Main();
    App::run(ui);
}

fn Main() -> impl Node {
    Window(WindowAttributes::default(), SingleQuadThingy())
}

fn SingleQuadThingy() -> impl LayoutItem {
    Resizable::new(Extent2::new(100.0, 100.0), |rect| {
        Quad::new(rect, Rgb::red())
    })
}
