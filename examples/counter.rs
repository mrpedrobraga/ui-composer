#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    let counter = Mutable::new(0);

    UIComposer::run_tui(Terminal(PanelContainer(center(Counter(counter)))))
}

fn Counter(counter: Mutable<i32>) -> impl TUI {
    let label = Label(counter.signal().map(|num| format!("Counter: {}", num)));
    let decr = Button(
        Label(always("Take 1".to_string())),
        counter.clone().effect(|e| *e -= 1),
    );
    let incr = Button(
        Label(always("Add 1".to_string())),
        counter.effect(|e| *e += 1),
    );

    flex(list![item(label), item(row((decr, incr)).with_gap(1.0))])
}

static TEXT_COLOR: Rgba<f32> = Rgba::new(156.0, 78.0, 10.0, 255.0);

fn Label(text_signal: impl Signal<Item = String> + Send + Sync) -> impl TUI {
    let text_signal = text_signal.broadcast();

    ItemBox::new(move |hx| {
        let text = text_signal
            .signal_ref(move |text| {
                Text()
                    .with_text(text.clone())
                    .with_rect(hx.rect)
                    .with_color(TEXT_COLOR / 255.0)
            })
            .into_blueprint();

        list![text]
    })
    .with_minimum_size(Extent2::new(15.0, 1.0))
}
