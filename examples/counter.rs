use futures_signals::signal::Signal;
use ui_composer::prelude::*;
use ui_composer::state::process::React;
use ui_composer::wgpu::components::{Button, Label};
use ui_composer::wgpu::pipeline::text::Text;
use ui_composer::wgpu::render_target::RenderDescriptor;
use ui_composer::Flex2;

fn main() {
    let text_state = Mutable::new(0);
    let text_state2 = Mutable::new(10);

    let c1 = Counter(text_state.clone());
    let c2 = Counter(text_state2);

    UIComposer::run(Window(Column(c1, c2)))
}

#[allow(non_snake_case)]
fn Counter(count_state: Mutable<i32>) -> impl LayoutItem<Content = impl RenderDescriptor> {
    let btn_sub = Button(Label("-"), count_state.clone().effect(|x| x - 1));
    let btn_add = Button(Label("+"), count_state.clone().effect(|x| x + 1));
    let label = ResizableItem::new(move |hints| {
        reactive_text(
            count_state.signal().map(|x| format!("Counter = {x}")),
            hints,
        )
    });

    Center(Flex2!( 3;
        [_] btn_sub,
        [1.0] label.with_minimum_size(Extent2::new(300.0, 24.0)),
        [_] btn_add,
    ))
}

fn reactive_text<S: Signal<Item = String> + Send>(
    s: S,
    hints: ParentHints,
) -> impl RenderDescriptor {
    React(s.map(move |x| Text(hints.rect, x, Rgb::new(1.0, 1.0, 1.0))))
}
