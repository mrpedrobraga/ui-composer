#![allow(non_snake_case)]
use ui_composer::prelude::*;
use ui_composer::wgpu::components::Label;
use ui_composer::wgpu::render_target::RenderDescriptor;

fn main() {
    UIComposer::run(Window(App()).with_title("Hello World".to_owned()));
}

fn App() -> impl LayoutItem<Content = impl RenderDescriptor> {
    Label("Welcome to UI Composer!\nHave fun and stay hydrated.")
}
