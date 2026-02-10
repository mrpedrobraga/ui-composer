#![allow(non_snake_case)]
use ui_composer::app::composition::layout::Resizable;
use ui_composer::prelude::{ItemBox, UIComposer};
use ui_composer::runners::tui::{Graphic, TUI, Terminal};
use ui_composer::standard::{flex, item};
use uix::uix as view;
use vek::{Extent2, Rgba};

fn main() {
    UIComposer::run_tui(view! (
        <Terminal>
            <flex>
                <item>
                    <Square>{ Rgba::red() }</Square>
                </item>
                <item grow=2.0>
                    <Square>{ Rgba::red() }</Square>
                </item>
                <item>
                    <Square>{ Rgba::red() }</Square>
                </item>
            </flex>
        </Terminal>
    ));
}

/// A simple coloured square.
fn Square(color: Rgba<f32>) -> impl TUI {
    view! (
        <ItemBox::new minimum_size=Extent2::new(16.0, 8.0)>
            @move |hx| <Graphic color=color rect=hx.rect />
        </ItemBox::new>
    )
}
