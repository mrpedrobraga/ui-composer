#![allow(non_snake_case)]
use lullaby_ui::layout::{flex, item};
use ui_composer::prelude::UIComposer;
use ui_composer_core::app::composition::layout::{ItemBox, Resizable};
use ui_composer_platform_tui::{Graphic, Terminal, TUI};
use uix::uix as view;
use vek::{Extent2, Rgba};

fn main() {
    UIComposer::run_tui(view! (
        <Terminal>
            <flex vertical_flow>
                <item grow=1.0>
                    <flex>
                        <item>
                            <Square>{ Rgba::magenta() }</Square>
                        </item>
                        <item grow=2.0>
                            <Square>{ Rgba::yellow() }</Square>
                        </item>
                        <item>
                            <Square>{ Rgba::cyan() }</Square>
                        </item>
                    </flex>
                </item>
                <item>
                    <Square>{ Rgba::blue() }</Square>
                </item>
            </flex>
        </Terminal>
    ));
}

/// A simple coloured square.
fn Square(color: Rgba<f32>) -> impl TUI {
    view! (
        <ItemBox::new minimum_size=Extent2::new(16.0, 2.0)>
            @move |hx| <Graphic color=color rect=hx.rect />
        </ItemBox::new>
    )
}
