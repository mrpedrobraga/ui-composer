use ui_composer::prelude::*;
use ui_composer::wgpu::components::Label;

fn main() {
    UIComposer::run(Window(Label("Hello, World!")))
}
