#![allow(unused, non_snake_case)]

use ui_composer::prelude::*;

fn main() {
    UIComposer::run(Window(App()).with_title("Click the Switch!"))
}

fn App() -> impl LayoutItem {
    Center(Switch(Mutable::new(false)))
}
