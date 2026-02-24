#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    UIComposer::run_tui(Terminal(app()))
}

fn app() -> impl Tui {
    let size = Extent2::new;
    view! (
        linewise_flow [
            inline ColorBox {size: size(6.0, 6.0), color: Rgba::red()},
            inline ColorBox {size: size(21.0, 8.0), color: Rgba::green()},
            MonospaceText (
                ("This is an amazing opportunity to show how cool layouting is!".to_string())
                (Rgba::white())
            )
            inline ColorBox {size: size(27.0, 2.0), color: Rgba::magenta()},
            inline ColorBox {size: size(12.0, 3.0), color: Rgba::blue()},
            inline ColorBox {size: size(15.0, 6.0), color: Rgba::red()},
            inline ColorBox {size: size(21.0, 10.0), color: Rgba::green()},
            inline ColorBox {size: size(6.0, 15.0), color: Rgba::magenta()},
            inline ColorBox {size: size(3.0, 5.0), color: Rgba::blue()},
            MonospaceText (
                ("What the hell?".to_string())
                (Rgba::white())
            )
            inline ColorBox {size: size(3.0, 8.0), color: Rgba::red()},
            inline ColorBox {size: size(9.0, 7.0), color: Rgba::green()},
            inline ColorBox {size: size(15.0, 5.0), color: Rgba::magenta()},
            inline ColorBox {size: size(12.0, 9.0), color: Rgba::blue()},
        ]
    )
}
