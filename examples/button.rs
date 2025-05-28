#![allow(non_snake_case)]
use ui_composer::{prelude::*, Component};

use ui_composer::winitwgpu::components::{Button, Label};

fn main() {
    let title_state = Mutable::new("Please click the button...".to_owned());

    UIComposer::run(
        Window(App(title_state.clone())).with_reactive_title(title_state.signal_cloned()),
    );
}

fn App(title_signal: Mutable<String>) -> Component!() {
    Center(Button(Label("Click me..."), move || {
        title_signal.set("Thank you.".to_owned())
    }))
}
