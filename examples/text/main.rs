#![allow(non_snake_case)]
use ui_composer::{prelude::*, standard::text::Text};

fn main() {
    AppBuilder::new(MyApp()).run();
}

fn MyApp() -> Text<&'static str> {
    Text("The quick brown fox jumped over the lazy dog. Sphinx of black quartz judge my vow.")
}
