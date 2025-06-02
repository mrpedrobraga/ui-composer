#![allow(non_snake_case)]
use ui_composer::winitwgpu::prelude::UI;
use ui_composer::{prelude::*, wgpu::components::Label};

fn main() {
    let app = Window(Main());

    UIComposer::run(app.with_title("My Beautiful App".into()))
}

fn Main() -> impl UI {
    Label("My Label")
}
