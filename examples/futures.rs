#![allow(non_snake_case)]

use chttp::ResponseExt;
use futures::FutureExt;
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use ui_composer::app::composition::effects::signal::SignalReactExt;
use ui_composer::app::composition::layout::{ItemBox, Resizable};
use ui_composer::prelude::UIComposer;
use ui_composer::runners::tui::nodes::Terminal;
use ui_composer::runners::tui::runner::TUIRunner;
use ui_composer::runners::tui::{Graphic, TUI};
use ui_composer::standard::runners::tui::render::text::Text;
use ui_composer::standard::Center;
use vek::{Extent2, Rgba};
use ui_composer::app::composition::effects::future::FutureReactExt;

fn main() {
    let some_state = Mutable::new("Loading...".to_string());
    let fut_state = some_state.clone();
    let fut = async move {
        let text = chttp::get_async(
            "https://baconipsum.com/api/?type=meat-and-filler&paras=1&format=text",
        )
            .await
            .expect("Bacon ipsum failed :-(")
            .text()
            .expect("Failed to parse response as text.");
        fut_state.set(text);
    };


    UIComposer::run_custom::<TUIRunner<_>>((
        fut.into_signal(),
        Terminal(Center(TestingFuture(some_state))),
    ))
}

fn TestingFuture(text_state: Mutable<String>) -> impl TUI {
    ItemBox::new(move |hx| {
        (
            Graphic {
                rect: hx.rect,
                color: Rgba::white(),
            },
            text_state
                .signal_cloned()
                .map(move |text| Text {
                    rect: hx.rect,
                    text,
                    color: Rgba::black(),
                })
                .react(),
        )
    })
    .with_minimum_size(Extent2::new(32.0, 16.0))
}
