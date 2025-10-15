use ui_composer::Flex2;
use ui_composer::prelude::*;
use ui_composer::state::process::SignalReactItem;
use ui_composer::wgpu::components::{Button, Label};
use ui_composer::winitwgpu::prelude::UI;

fn main() {
    let text_state = Mutable::new(0);

    let c1 = Counter(text_state.clone());
    let c2 = Counter(text_state);

    UIComposer::run(Window(Column(c1, c2)))
}

#[allow(non_snake_case)]
fn Counter(count_state: Mutable<i32>) -> impl UI {
    let btn_sub = Button(Label("-"), count_state.clone().effect(|x| x - 1));
    let btn_add = Button(Label("+"), count_state.clone().effect(|x| x + 1));
    let label = count_state
        .signal()
        .map(|x| format!("Counter = {x}"))
        .map(Label);

    Center(Flex2!( 2;
        [_] btn_sub,
        //[1.0] React(label),
        [_] btn_add,
    ))
}
