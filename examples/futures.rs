#![allow(non_snake_case)]
use futures::FutureExt;
use {chttp::ResponseExt, lullaby_ui::components::Ui};
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    UIComposer::run_tui(Terminal(center(TestingFuture())))
}

fn TestingFuture() -> impl Ui {
    ItemBox::new(|hx| {
        let fut = chttp::get_async(
            "https://baconipsum.com/api/?type=meat-and-filler&paras=1&format=text",
        )
        .then(|res| async {
            res.expect("Bacon ipsum failed :-(")
                .text()
                .expect("Failed to parse response as text.")
        });

        (
            Graphic {
                rect: hx.rect,
                color: Srgba::new(1.0, 1.0, 1.0, 1.0),
            },
            fut.map(move |text| Text {
                rect: hx.rect.inflate(Size2::new(-1.0, 0.0)),
                text,
                color: Srgba::new(1.0, 0.0, 0.0, 1.0),
            })
            .into_blueprint(),
        )
    })
    .with_minimum_size(Size2::new(64.0, 16.0))
}
