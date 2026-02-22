#![allow(non_snake_case)]
use lullaby_ui::layout::{center, flex, item, row};
use lullaby_ui::text::Text;
use ui_composer::list;
use ui_composer::prelude::UIComposer;
use ui_composer_basic_ui::items::{Hover, Tap};
use ui_composer_core::app::composition::effects::signal::SignalReactExt;
use ui_composer_core::app::composition::layout::hints::ParentHints;
use ui_composer_core::app::composition::layout::{ItemBox, Resizable as _};
use ui_composer_geometry::RectExt;
use ui_composer_platform_tui::TUI;
use ui_composer_platform_tui::{Graphic, Terminal};
use ui_composer_state::effect::Effect;
use ui_composer_state::futures_signals::signal::{always, Mutable};
use ui_composer_state::futures_signals::signal::{Signal, SignalExt};
use ui_composer_state::State;
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

static BUTTON_COLOR: Rgba<f32> = Rgba::new(255.0, 217.0, 179.0, 255.0);
static BUTTON_COLOR_HOVER: Rgba<f32> = Rgba::new(235.0, 189.0, 143.0, 255.0);
static BUTTON_TEXT_COLOR: Rgba<f32> = Rgba::new(175.0, 90.0, 16.0, 255.0);
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

fn Button(mut label: impl TUI, effect: impl Effect + 'static) -> impl TUI {
    let is_hovered: Mutable<bool> = Mutable::default();

    ItemBox::new(move |hx| {
        let hover = Hover::new(hx.rect, is_hovered.clone());
        let tap = Tap::new(hx.rect, effect.clone());

        let rect = is_hovered
            .signal_ref(move |is_hovered| {
                if *is_hovered {
                    Graphic::new(hx.rect, BUTTON_COLOR_HOVER / 255.0)
                } else {
                    Graphic::new(hx.rect, BUTTON_COLOR / 255.0)
                }
            })
            .react();
        let label = label.lay(ParentHints {
            rect: hx.rect.expand_from_center(-1.0, -1.0, -1.0, -1.0),
            ..hx
        });

        list![hover, tap, rect, label]
    })
    .with_minimum_size(Extent2::new(15.0, 3.0))
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
