#![allow(non_snake_case)]
use lullaby_ui::components::button::Button;
use lullaby_ui::layout::{center, flex, item, row};
use lullaby_ui::primitives::graphic::Graphic;
use lullaby_ui::primitives::text::Text;
use ui_composer::list;
use ui_composer::prelude::UIComposer;
use ui_composer_core::app::composition::effects::signal::SignalReactExt;
use ui_composer_core::app::composition::layout::{ItemBox, Resizable as _};
use ui_composer_platform_tui::TUI;
use ui_composer_platform_tui::Terminal;
use ui_composer_state::State;
use ui_composer_state::futures_signals::signal::{Mutable, always};
use ui_composer_state::futures_signals::signal::{Signal, SignalExt};
use vek::{Extent2, Rgba};

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

static SURFACE_COLOR: Rgba<f32> = Rgba::new(255.0, 253.0, 248.0, 255.0);
static SURFACE_COLOR_2: Rgba<f32> = Rgba::new(255.0, 241.0, 231.0, 255.0);

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
            .react();

        list![text]
    })
    .with_minimum_size(Extent2::new(15.0, 1.0))
}

fn PanelContainer(mut child: impl TUI) -> impl TUI {
    let min_size = child.get_minimum_size();

    ItemBox::new(move |hx| {
        let rect = Graphic::new(hx.rect, SURFACE_COLOR / 255.0);
        let c = child.lay(hx);

        list![rect, c]
    })
    .with_minimum_size(min_size)
}
