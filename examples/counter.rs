#![allow(non_snake_case)]
use lullaby_ui::layout::{flex, item, Center};
use lullaby_ui::text::Text;
use ui_composer::list;
use ui_composer::prelude::UIComposer;
use ui_composer_core::app::composition::effects::signal::SignalReactExt;
use ui_composer_core::app::composition::layout::{ItemBox, Resizable as _};
use ui_composer_core::items::{Hover, Tap};
use ui_composer_platform_tui::TUI;
use ui_composer_platform_tui::{Graphic, Terminal};
use ui_composer_state::effect::Effect;
use ui_composer_state::futures_signals::signal::{always, Mutable};
use ui_composer_state::futures_signals::signal::{Signal, SignalExt};
use ui_composer_state::State;
use vek::{Extent2, Rgba};

fn main() {
    let counter = Mutable::new(0);

    UIComposer::run_tui(Terminal(Center(Counter(counter))))
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

    flex(list![item(label), item(decr), item(incr)])
}

fn Label(text_signal: impl Signal<Item = String> + Send + Sync) -> impl TUI {
    let text_signal = text_signal.broadcast();

    ItemBox::new(move |hx| {
        let text = text_signal
            .signal_ref(move |text| {
                Text()
                    .with_text(text.clone())
                    .with_rect(hx.rect)
                    .with_color(Rgba::white())
            })
            .react();

        list![text]
    })
    .with_minimum_size(Extent2::new(15.0, 1.0))
}

fn Button(mut label: impl TUI, effect: impl Effect + 'static) -> impl TUI {
    let is_hovered: Mutable<bool> = Mutable::default();

    ItemBox::new(move |hx| {
        let hover = Hover::new(hx.rect, is_hovered.clone());
        let tap = Tap::new(hx.rect, effect.clone());

        let rect = is_hovered
            .signal_ref(move |is_hovered| {
                if *is_hovered {
                    Graphic::new(hx.rect, Rgba::gray(0.5))
                } else {
                    Graphic::new(hx.rect, Rgba::gray(0.2))
                }
            })
            .react();
        let label = label.lay(hx);

        list![hover, tap, rect, label]
    })
    .with_minimum_size(Extent2::new(15.0, 1.0))
}
