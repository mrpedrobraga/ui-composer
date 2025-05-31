#![allow(unused, non_snake_case)]

use ui_composer::prelude::*;
use ui_composer::wgpu::render_target::RenderDescriptor;

use ui_composer::wgpu::components::{Label, Switch};

fn main() {
    UIComposer::run(Window(App()).with_title("Click the Switch!".into()))
}

fn App() -> impl LayoutItem<Content = impl RenderDescriptor> {
    let state_a = Mutable::new(false);
    let state_b = Mutable::new(false);

    Center(Row(
        Column(Square(state_a.clone()), Square(state_a)),
        Column(Square(state_b.clone()), Square(state_b)),
    ))
}

fn Square(state: Mutable<bool>) -> impl LayoutItem<Content = impl RenderDescriptor> {
    WithSize(
        Extent2::new(64.0, 32.0),
        Center(Row(Label("Hello, there!"), Center(Switch(state)))),
    )
}
