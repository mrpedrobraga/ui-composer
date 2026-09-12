#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    let counter = Mutable::new(0);

    UIComposer::run_tui(Terminal(PanelContainer(center(Counter(counter)))))
}

fn Counter(counter: Mutable<i32>) -> impl Tui {
    let txt_count =
        ReactiveLabel(counter.signal().map(|num| format!("Counter: {}", num)));
    let btn_decrement = Button(Label("Take 1"), counter.clone().effect(|e| *e -= 1));
    let btn_increment = Button(Label("Add 1"), counter.effect(|e| *e += 1));

    view! {
        flex [
            item ((btn_decrement))
            item center ((txt_count))
            item ((btn_increment))
        ]
    }
}

fn ReactiveLabel(
    text_signal: impl Signal<Item = String> + Send + Sync,
) -> impl Tui {
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
