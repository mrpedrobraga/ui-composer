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
    let size = Extent2::new(w, h / 2).as_() / 60.0;

    view! {
        center with_size {size: Extent2::new(100.0, 20.0)} linewise_flow [
            MonospaceText(("Look at this image.".to_string()) (Rgba::white()))
            inline Image {resized: size} ((im))
            MonospaceText(("Cool, right?".to_string()) (Rgba::white()))
        ]
    }
}
