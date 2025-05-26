#![allow(non_snake_case)]

use async_std::path::Path;
use futures::future::FutureExt as _;
use futures_time::future::FutureExt as _;
use futures_time::time::Duration;
use serde::Deserialize;
use std::future::Future;
use ui_composer::prelude::*;

fn main() {
    UIComposer::run(
        Window(Center(Row(
            PersonView("./examples/futures/martha.json"),
            PersonView("./examples/futures/pedro.json"),
        )))
        .with_title("Futures"),
    );
}

#[derive(Debug, Clone, Deserialize)]
struct Person {
    name: String,
    y: f32,
    color: Rgb<f32>,
}

fn PersonView<P>(path: P) -> impl LayoutItem
where
    P: AsRef<Path> + Send + 'static,
{
    let person_state = Mutable::new(None);

    let person_fut = fetch_person_and_put_in(path, person_state.clone());
    std::thread::spawn(move || pollster::block_on(person_fut));

    ResizableItem::new(move |hx| {
        let person_square = person_state
            .signal_cloned()
            .map(move |person_opt| {
                if let Some(person) = person_opt {
                    hx.rect
                        .translated(Vec2::unit_y() * -person.y)
                        .with_color(person.color)
                } else {
                    hx.rect.with_color(Rgb::gray(0.5))
                }
            })
            .process();

        person_square
    })
    .with_minimum_size(Extent2::new(100.0, 100.0))
}

fn fetch_person_and_put_in<P>(path: P, state: Mutable<Option<Person>>) -> impl Future<Output = ()>
where
    P: AsRef<Path>,
{
    let person_fut = async_std::fs::read_to_string(path)
        .map(|text| serde_json::from_str::<Person>(&text.unwrap()).unwrap())
        .map(move |person| {
            state.set(Some(person.clone()));
        })
        .delay(Duration::from_secs(1));
    person_fut
}
