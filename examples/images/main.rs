use lullaby_ui::image::{self, GenericImageView};
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    UIComposer::run_tui(Terminal(app()));
}

fn app() -> impl Ui {
    let im = std::sync::Arc::new(
        image::open("./examples/images/assets/castle_sprite.png").unwrap(),
    );
    let (w, h) = im.dimensions();
    let size = Size2::new(w as f32, h as f32 / 2.0) / 60.0f32;

    view! {
        center with_size {size: Size2::new(100.0, 20.0)} linewise_flow [
            MonospaceText(("Look at this image.".to_string()) (Srgba::new(1.0, 1.0, 1.0, 1.0)))
            inline Image {resized: size} ((im))
            MonospaceText(("Cool, right?".to_string()) (Srgba::new(1.0, 1.0, 1.0, 1.0)))
        ]
    }
}
