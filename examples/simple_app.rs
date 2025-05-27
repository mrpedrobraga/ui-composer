#![allow(non_snake_case)]
use ui_composer::prelude::*;

fn main() {
    UIComposer::run(Window(App()).with_title("My Beautiful App"))
}

fn App() -> impl LayoutItem {
    Label("My Label")
}
