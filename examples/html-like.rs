#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

static SURFACE_COLOR: Srgba = Srgba::new(255.0, 253.0, 248.0, 255.0);
static SURFACE_COLOR_2: Srgba = Srgba::new(255.0, 241.0, 231.0, 255.0);

static BUTTON_COLOR: Srgba = Srgba::new(255.0, 217.0, 179.0, 255.0);
static BUTTON_COLOR_HOVER: Srgba = Srgba::new(235.0, 189.0, 143.0, 255.0);

fn main() {
    UIComposer::run_tui(uix! (
        <Terminal>
            <flex vertical_flow>
                <item grow=1.0>
                    <flex>
                        <item>
                            <Square>{ SURFACE_COLOR / 255.0 }</Square>
                        </item>
                        <item grow=2.0>
                            <Square>{ SURFACE_COLOR_2 / 255.0 }</Square>
                        </item>
                        <item>
                            <Square>{ BUTTON_COLOR / 255.0 }</Square>
                        </item>
                    </flex>
                </item>
                <item>
                    <Square>{ BUTTON_COLOR_HOVER / 255.0 }</Square>
                </item>
            </flex>
        </Terminal>
    ));
}

/// A simple coloured square.
fn Square(color: Srgba) -> impl Tui {
    uix! (
        <ItemBox::new minimum_size=Size2::new(16.0, 2.0)>
            @move |hx| <Graphic color=color rect=hx.rect />
        </ItemBox::new>
    )
}
