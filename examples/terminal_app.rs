#![allow(non_snake_case)]
use ui_composer::geometry::layout::{ItemBox, Resizable};
use ui_composer::runners::tui::nodes::{Terminal};
use ui_composer::runners::tui::runner::{TUIEnvironment, TUIRunner};
use ui_composer::runners::tui::{Graphic, TUI};
use ui_composer::standard::prelude::UIComposer;
use vek::{Extent2, Rgba};
use ui_composer::app::composition::elements::Blueprint;
use ui_composer::Flex;

fn main() {
    UIComposer::run_custom::<TUIRunner<_>>(AApp())
}

fn AApp() -> impl Blueprint<TUIEnvironment> {
    Terminal(App())
}

fn App() -> impl TUI {
    //Center(
    Flex! { 2;
        [1.0] Flex! { 3;
                [_] Square(Rgba::red()),
                [1.0] Square(Rgba::green()),
                [_] Square(Rgba::blue()),
            },
        [_] Square(Rgba::yellow()),
    }.with_vertical_flow()
    //)
}

fn Square(color: Rgba<f32>) -> impl TUI {
    ItemBox::new(move |hx| Graphic {
        rect: hx.rect,
        color,
    }).with_minimum_size(Extent2::new(16.0, 8.0))
}