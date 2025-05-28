#![allow(unused, non_snake_case)]

use ui_composer::prelude::*;
use ui_composer::winitwgpu::render_target::Render;
use ui_composer::Flex;

use ui_composer::winitwgpu::components::Switch;

fn main() {
    UIComposer::run(Window(App()).with_title("Click the Switch!"))
}

fn App() -> impl LayoutItem<Content = impl Render> {
    let state_a = Mutable::new(false);
    let state_b = Mutable::new(false);

    Center(Row(
        Column(Square(state_a.clone()), Square(state_a)),
        Column(Square(state_b.clone()), Square(state_b)),
    ))
}

fn Square(state: Mutable<bool>) -> impl LayoutItem<Content = impl Render> {
    WithSize(Extent2::new(64.0, 32.0), Center(Switch(state)))
}
