#![allow(non_snake_case)]

use futures_time::{future::FutureExt, time::Duration};
use serde::Deserialize;
use ui_composer::prelude::*;

fn main() {
    UIComposer::run(
        Window(Center(PersonView("https://mrpedrobraga.com/api"))).with_title("Futures"),
    );
}

#[derive(Debug, Clone, Deserialize)]
struct Person {
    #[allow(unused)]
    name: String,
    age: i32,
}

fn PersonView(uri: &'static str) -> impl LayoutItem {
    let person_state = Mutable::new(None);

    let person_fetch_process = fetch_person_and_put_in(uri, person_state.clone()).delay(Duration::from_secs(1));
    std::thread::spawn(move || futures::executor::block_on(person_fetch_process));

    ResizableItem::new(move |hx| {
        let person_square = person_state
            .signal_cloned()
            .map(move |person_opt| {
                if let Some(person) = person_opt {
                    hx.rect
                        .translated(-Vec2::unit_y() * (person.age as f32))
                        .with_color(Rgb::new(0.5, 0.6, 0.9))
                } else {
                    hx.rect.with_color(Rgb::gray(0.5))
                }
            })
            .process();

        person_square
    })
    .with_minimum_size(Extent2::new(100.0, 100.0))
}

async fn fetch_person_and_put_in(uri: &'static str, state: Mutable<Option<Person>>) {
    use chttp::prelude::*;

    let person_raw = chttp::get_async(uri).await.unwrap().text().unwrap();
    let person: Person = serde_json::from_str(&person_raw).unwrap();
    state.set(Some(person));
}