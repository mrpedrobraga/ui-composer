#![allow(non_snake_case)]
use ui_composer::wgpu::render_target::Render;
use ui_composer::{prelude::*, wgpu::components::Label};

fn main() {
    UIComposer::run(Window(App()).with_title("My Beautiful App".into()))
}

fn App() -> impl LayoutItem<Content = impl Render> {
    Label("My Label")
}
