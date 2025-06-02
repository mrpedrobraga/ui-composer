use ui_composer::prelude::items::Typing;
use ui_composer::prelude::process::React;
use ui_composer::prelude::*;
use ui_composer::wgpu::components::*;
use ui_composer::winitwgpu::prelude::*;
use ui_composer::Flex2;

fn main() {
    let state = Mutable::new(String::new());

    let print_effect = {
        let state = state.clone();
        move || println!("{}", state.get_cloned())
    };

    UIComposer::run(Window(
        Flex2! { 2;
            [1.0] TextEdit(state),
            [_] Button(Label("Print"), print_effect),
        }
        .with_vertical_flow(),
    ));
}

fn TextEdit(state: Mutable<String>) -> impl UI {
    ResizableItem::new(move |hints| {
        let ts = state.clone();
        let sig = ts.signal_cloned();

        let typing = Typing::new(ts);

        React(sig.map(move |text| (typing.clone(), Label(text).lay(hints))))
    })
    .with_minimum_size(Extent2::new(100.0, 200.0))
}
