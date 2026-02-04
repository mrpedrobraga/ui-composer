use ui_composer::experimental::into_ui::IntoDefaultUI;
use ui_composer::standard::prelude::*;

fn main() {
    UIComposer::run(Window(App().into_default_ui()));
}

fn App() -> impl IntoDefaultUI {
    ("Hello, world!", "Also hello!")
}