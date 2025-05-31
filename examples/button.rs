use ui_composer::prelude::*;
use ui_composer::wgpu::components::{Button, Label};

fn main() {
    UIComposer::run(Window(Button(Label("Please click me!"), || ())))
}
