#![allow(non_snake_case)]

use lullaby_ui::layout::{center, row};
use lullaby_ui::primitives::graphic::Graphic;
use lullaby_ui::primitives::text::Text;
use ui_composer::list;
use ui_composer::prelude::UIComposer;
use ui_composer_basic_ui::items::{Hover, Typing};
use ui_composer_core::app::composition::effects::signal::SignalReactExt;
use ui_composer_core::app::composition::layout::{ItemBox, Resizable};
use ui_composer_platform_tui::TUI;
use ui_composer_platform_tui::nodes::Terminal;
use ui_composer_state::futures_signals::signal::{Mutable, SignalExt as _};
use ui_composer_view_macro::view;
use vek::{Extent2, Rgba};

fn main() {
    UIComposer::run_tui(view! {
        Terminal {
            center row(gap=2.0) [
                TestingTyping,
                TestingHover,
            ]
        }
    });
}

fn TestingTyping() -> impl TUI {
    let string_state: Mutable<String> = Mutable::default();

    ItemBox::new(move |hx| {
        let typing = Typing::new(string_state.clone());

        let square = Graphic {
            rect: hx.rect,
            color: Rgba::new(0.4, 0.5, 0.7, 1.0),
        };

        let label = string_state
            .signal_cloned()
            .map(move |text| Text {
                rect: hx.rect,
                color: Rgba::black(),
                text,
            })
            .react();

        list![typing, square, label]
    })
    .with_minimum_size(Extent2::new(32.0, 12.0))
}

fn TestingHover() -> impl TUI {
    let hover_state: Mutable<bool> = Mutable::default();

    ItemBox::new(move |hx| {
        let hover = Hover::new(hx.rect, hover_state.clone());

        let square_factory = move |is_hovered| {
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
        };

        let square = hover_state.signal().map(square_factory).react();

        list![hover, square]
    })
    .with_minimum_size(Extent2::new(32.0, 16.0))
}
