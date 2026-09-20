#![allow(non_snake_case)]
use futures::FutureExt;
use {chttp::ResponseExt, lullaby_ui::components::Ui};
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    UIComposer::run_tui(Terminal(center(TestingFuture())))
}

fn TestingFuture() -> impl Ui {
    let fut =
        chttp::get_async("https://baconipsum.com/api/?type=meat-and-filler&paras=1&format=text")
            .then(|res| async {
                res.expect("Bacon ipsum failed :-(")
                    .text()
                    .expect("Failed to parse response as text.")
            });
    let fut = fut
        .map(move |text| Text {
            rect: Rect::new(Point2::new(0.0, 0.0), Size2::new(10.0, 10.0)),
            text,
            color: Srgba::new(1.0, 0.0, 0.0, 1.0),
        })
        .shared()
        .into_blueprint();

    ItemBox::new(move |hx| {
        (
            Graphic {
                rect: hx.rect,
                color: Srgba::new(1.0, 1.0, 1.0, 1.0),
            },
            fut.clone(),
        )
    })
    .with_minimum_size(Size2::new(64.0, 16.0))
}
