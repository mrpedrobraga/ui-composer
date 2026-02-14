#![allow(non_snake_case)]

use chttp::ResponseExt;
use futures::FutureExt;
use ui_composer::app::composition::effects::future::FutureReactExt;
use ui_composer::app::composition::layout::{ItemBox, Resizable};
use ui_composer::geometry::RectExt;
use ui_composer::prelude::UIComposer;
use ui_composer::runners::tui::nodes::Terminal;
use ui_composer::runners::tui::runner::TUIRunner;
use ui_composer::runners::tui::{Graphic, TUI};
use ui_composer::standard::runners::tui::render::text::Text;
use ui_composer::standard::Center;
use vek::{Extent2, Rgba};

fn main() {
    UIComposer::run_custom::<TUIRunner<_>>(Terminal(Center(TestingFuture())))
}

fn TestingFuture() -> impl TUI {
    ItemBox::new(move |hx| {
        let fut =
            chttp::get_async(
                "https://baconipsum.com/api/?type=meat-and-filler&paras=1&format=text",
            ).then(|res| async {
                res.expect("Bacon ipsum failed :-(")
                    .text()
                    .expect("Failed to parse response as text.")
            });

        (
            Graphic {
                rect: hx.rect,
                color: Rgba::white(),
            },
            fut.map(move |text| Text {
                rect: hx.rect.expand_from_center(-1.0, -1.0, 0.0, 0.0),
                text,
                color: Rgba::red(),
            }).into_signal(),
        )
    })
    .with_minimum_size(Extent2::new(32.0, 16.0))
}
