#![allow(non_snake_case)]
use ui_composer::{prelude::*, winitwgpu::components::Label};

fn main() {
    UIComposer::run(Window(App()).with_title("My Beautiful App"))
}

fn App() -> impl LayoutItem {
    Label("My Label")
}
