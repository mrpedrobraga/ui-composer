#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    let counter = Mutable::new(0);

    UIComposer::run_tui(Terminal(PanelContainer(center(Counter(counter)))))
}

fn Counter(counter: Mutable<i32>) -> impl TUI {
    let label =
        ReactiveLabel(counter.signal().map(|num| format!("Counter: {}", num)));
    let decr = Button(Label("Take 1"), counter.clone().effect(|e| *e -= 1));
    let incr = Button(Label("Add 1"), counter.effect(|e| *e += 1));

    view! {
        flex [
            item {{decr}}
            item {{label}}
            item {{incr}}
        ]
    }
}

fn ReactiveLabel(
    text_signal: impl Signal<Item = String> + Send + Sync,
) -> impl TUI {
    let text_signal = text_signal.broadcast();

    ItemBox::new(move |hx| {
        let text = text_signal
            .signal_ref(move |text| Label(text.clone()).lay(hx))
            .into_blueprint();

        list![text]
    })
    .with_minimum_size(Extent2::new(15.0, 1.0))
}
