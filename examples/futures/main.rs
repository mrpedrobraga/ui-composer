#![allow(non_snake_case)]

use futures_time::future::FutureExt as _;
use futures::future::FutureExt as _;
use futures_time::time::Duration;
use pollster::block_on;
use serde::Deserialize;
use ui_composer::prelude::*;

fn main() {
    App::run(Window(Center(Main())).with_title("Futures"));
}

fn Main() -> impl LayoutItem {
    let person_state: Editable<Option<Person>> = Editable::new(None);

    let person_state_ = person_state.clone();
    let person_fut =
        async_std::fs::read_to_string("./examples/futures/data.json")
            .map(|text| {
                serde_json::from_str::<Person>(&text.unwrap()).unwrap()
            })
            .map(move |person: Person| {
               person_state_.set(Some(person))
            })
            .delay(Duration::from_secs(1));
    std::thread::spawn(move || {
        block_on(person_fut);
    });

    Resizable::new(move |hx| {
        person_state.signal_cloned().map(move |person_opt| {
            person_opt.map(move |person| {
                hx.rect.translated(Vec2::unit_y() * -person.y).with_color(person.color)
            })
        }).process()
    }).with_minimum_size(Extent2::new(100.0, 100.0))
}

#[derive(Debug, Clone, Deserialize)]
struct Person {
    name: String,
    y: f32,
    color: Rgb<f32>
}