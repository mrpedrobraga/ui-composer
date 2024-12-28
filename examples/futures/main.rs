#![allow(non_snake_case)]

use futures::future::FutureExt as _;
use futures_time::future::FutureExt as _;
use futures_time::time::Duration;
use serde::Deserialize;
use ui_composer::prelude::*;

fn main() {
    UIComposer::run(Window(Center(Row(Main(), Main()))).with_title("Futures"));
}

fn Main() -> impl LayoutItem {
    let person_state: Mutable<Option<Person>> = Mutable::new(None);

    let person_state_ = person_state.clone();
    let person_fut = async_std::fs::read_to_string("./examples/futures/data.json")
        .map(|text| serde_json::from_str::<Person>(&text.unwrap()).unwrap())
        // TODO: Transform this into a new function, similar to `.animate()` that returns a future that can be processed.
        .map(move |person: Person| person_state_.set(Some(person)))
        .delay(Duration::from_secs(1));

    std::thread::spawn(move || pollster::block_on(person_fut));

    ResizableItem::new(move |hx| {
        let person_square = person_state
            .signal_cloned()
            .map(move |person_opt| {
                person_opt
                    .map(move |person| {
                        hx.rect
                            .translated(Vec2::unit_y() * -person.y)
                            .with_color(person.color)
                    })
                    .or(Some(hx.rect.with_color(Rgb::new(0.7, 0.7, 0.7))))
            })
            .process();

        (person_square)
    })
    .with_minimum_size(Extent2::new(100.0, 100.0))
}

#[derive(Debug, Clone, Deserialize)]
struct Person {
    name: String,
    y: f32,
    color: Rgb<f32>,
}
