#![allow(non_snake_case)]

use futures::FutureExt as _;
use futures_time::future::FutureExt as _;
use futures_time::time::Duration;
use serde::Deserialize;
use ui_composer::prelude::process::Await;
use ui_composer::prelude::*;
use ui_composer::wgpu::components::Label;
use ui_composer::wgpu::render_target::RenderDescriptor;

fn main() {
    UIComposer::run(Window(Center(App())).with_title("Futures".into()));
}

#[derive(Debug, Clone, Deserialize)]
struct Person {
    #[allow(unused)]
    name: String,
    age: i32,
}

async fn fetch_person(uri: &'static str) -> Person {
    use chttp::prelude::*;
    let person_raw = chttp::get_async(uri).await.unwrap().text().unwrap();
    let person: Person = serde_json::from_str(&person_raw).unwrap();
    person
}

fn App() -> impl LayoutItem<Content = impl RenderDescriptor> {
    ResizableItem::new(|hints| {
        let person_fut = fetch_person("https://mrpedrobraga.com/api");

        let ui_fut = person_fut
            .delay(Duration::from_secs(1))
            .map(move |person| PersonView(person).lay(hints));

        Await(ui_fut)
    })
    .with_minimum_size(Extent2::new(200.0, 200.0))
}

fn PersonView(person: Person) -> impl LayoutItem<Content = impl RenderDescriptor> {
    Label(format!("{}, {} years old.", person.name, person.age))
}
