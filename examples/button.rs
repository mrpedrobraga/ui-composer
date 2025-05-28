#![allow(non_snake_case)]
use ui_composer::{prelude::*, Component};

use ui_composer::winitwgpu::components::{Button, Label};

fn main() {
    UIComposer::run(Window(App()));
}

fn App() -> Component!() {
    Center(Button(Label("Click me..."), || println!("Hello, there!")))
}
