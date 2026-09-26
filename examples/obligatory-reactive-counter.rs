#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    let counter = Mutable::new(0);

    UIComposer::run_tui(Terminal(PanelContainer(Counter(counter))))
}

fn Counter(count: Mutable<i32>) -> impl Tui {
    let txt_count = ReactiveLabel(count.signal_ref(|num| format!("Counter: {num}")));

    view! {
        center flex [
            item Button (
                Label (("Take 1"))
                (count.clone().effect(|e| *e -= 1))
            )
            item {grow: 1.0} center ((txt_count))
            item Button (
                Label (("Add 1"))
                (count.effect(|e| *e += 1))
            )
        ]
    }
}

fn ReactiveLabel(text_signal: impl Signal<Item = String> + Send + Sync) -> impl Tui {
    let text_signal = text_signal.broadcast();

    ItemBox::new(move |hx| {
        let text = text_signal
            .signal_ref(move |text| {
                let mut l = Label(text.clone());
                l.prepare(hx);
                l.place(hx)
            })
            .into_blueprint();

        list![text]
    })
    .with_minimum_size(Size2::new(15.0, 1.0))
}
