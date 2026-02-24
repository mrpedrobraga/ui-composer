use lullaby_ui::image::{self, GenericImageView};
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    UIComposer::run_tui(Terminal(app()));
}

fn app() -> impl Ui {
    let im = image::open("./examples/images/assets/castle_sprite.png").unwrap();
    let (w, h) = im.dimensions();
    let size = Extent2::new(w, h / 2).as_() / 10.0;

    view! {
        center Image {resized: size} ((im))
    }
}
