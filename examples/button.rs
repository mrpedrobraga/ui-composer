#![allow(non_snake_case)]
use ui_composer::{prelude::*, Component};

use ui_composer::winitwgpu::components::{Button, Label};

fn main() {
    let title_signal = Mutable::new(String::from("Please click the button..."));

    UIComposer::run(Window(App(title_signal.clone())).with_reactive_title(title_signal));
}

fn App(title_signal: Mutable<String>) -> Component!() {
    Center(Button(Label("Click me..."), move || {
        title_signal.set("Thank you.".into())
    }))
}
