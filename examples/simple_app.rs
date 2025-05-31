#![allow(non_snake_case)]
use ui_composer::wgpu::render_target::RenderDescriptor;
use ui_composer::{prelude::*, wgpu::components::Label};

fn main() {
    let app = Window(Main());

    UIComposer::run(app.with_title("My Beautiful App".into()))
}

fn Main() -> impl LayoutItem<Content = impl RenderDescriptor> {
    Label("My Label")
}
