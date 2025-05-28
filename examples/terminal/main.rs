use ui_composer::{prelude::*, tui::*, UI};

fn main() {
    let app = Terminal(Main());
    UIComposer::run_custom::<TUIBackend<_>>(app);
}

#[allow(non_snake_case)]
fn Main() -> UI!(terminal) {
    ResizableItem::new(|hints| Graphic::from(hints.rect).with_color(Rgba::new(0.5, 0.7, 0.8, 1.0)))
        .with_minimum_size(Extent2::new(32.0, 32.0))
}
