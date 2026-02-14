#![allow(non_snake_case)]

use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use ui_composer::app::composition::effects::signal::SignalReactExt;
use ui_composer::app::composition::layout::{ItemBox, Resizable};
use ui_composer::list;
use ui_composer::prelude::{Hover, Row, Typing, UIComposer};
use ui_composer::runners::tui::nodes::Terminal;
use ui_composer::runners::tui::render::text::Text;
use ui_composer::runners::tui::runner::TUIRunner;
use ui_composer::runners::tui::{Graphic, TUI};
use ui_composer::standard::Center;
use vek::{Extent2, Rgba};

fn main() {
    UIComposer::run_custom::<TUIRunner<_>>(Terminal(Center(Row(
        TestingTyping(),
        TestingHover(),
    ))))
}

fn TestingTyping() -> impl TUI {
    let string_state = Mutable::new(String::new());

    ItemBox::new(move |hx| {
        let typing = Typing::new(string_state.clone());

        list!(
            typing,
            Graphic {
                rect: hx.rect,
                color: Rgba::new(0.4, 0.5, 0.7, 1.0)
            },
            string_state
                .signal_cloned()
                .map(move |text| Text {
                    rect: hx.rect,
                    color: Rgba::black(),
                    text
                })
                .react(),
        )
    })
    .with_minimum_size(Extent2::new(32.0, 12.0))
}

fn TestingHover() -> impl TUI {
    let hover_state = Mutable::new(false);

    ItemBox::new(move |hx| {
        let hover = Hover::new(hx.rect, hover_state.clone());

        list!(
            hover,
            hover_state
                .signal()
                .map(move |is_hovered| {
                    (
                        Graphic {
                            rect: hx.rect,
                            color: if is_hovered {
                                Rgba::yellow()
                            } else {
                                Rgba::white()
                            },
                        },
                        Text {
                            rect: hx.rect,
                            color: Rgba::black(),
                            text: if is_hovered {
                                String::from("Is hovered!")
                            } else {
                                String::from("Hover me...")
                            },
                        },
                    )
                })
                .react(),
        )
    })
    .with_minimum_size(Extent2::new(32.0, 16.0))
}
