#![allow(non_snake_case)]
use ui_composer::{prelude::*, standard::text::Text};

fn main() {
    AppBuilder::new(MyApp()).run();
}

fn MyApp() -> Text<&'static str> {
    Text("This is an incredibly cool text.")
}
