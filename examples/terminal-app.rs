#![allow(non_snake_case)]
use ui_composer::app::composition::layout::{ItemBox, Resizable};
use ui_composer::list;
use ui_composer::prelude::{flex, UIComposer};
use ui_composer::runners::tui::nodes::Terminal;
use ui_composer::runners::tui::runner::TUIRunner;
use ui_composer::runners::tui::{Graphic, TUI};
use ui_composer::standard::item;
use ui_composer::standard::runners::tui::render::text::Text;
use vek::{Extent2, Rgba};

fn main() {
    UIComposer::run_custom::<TUIRunner<_>>(Terminal(flex(list!(
        item(Square(Rgba::new(255, 235, 252, 255).as_() / 255.0)).with_grow(2.0),
        item(Square(Rgba::new(245, 219, 241, 255).as_() / 255.0)).with_grow(1.0),
        item(Square(Rgba::new(134, 70, 139, 255).as_() / 255.0)).with_grow(0.0),
    ))))
}

fn Square(color: Rgba<f32>) -> impl TUI {
    ItemBox::new(move |hx| {
        (
            Graphic {
                rect: hx.rect,
                color,
            },
            Text {
                rect: hx.rect,
                text: "Hi there, my name is Pedro!".to_string(),
                color: Rgba::black(),
            },
        )
    })
    .with_minimum_size(Extent2::new(16.0, 4.0))
}
