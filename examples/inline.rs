#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    UIComposer::run_tui(Terminal(center(app())))
}

fn app() -> impl Tui {
    let size = Size2::new;

    let white = Srgba::new(1.0, 1.0, 1.0, 1.0);
    let red = Srgba::new(1.0, 0.0, 0.0, 1.0);
    let green = Srgba::new(0.0, 1.0, 0.0, 1.0);
    let blue = Srgba::new(0.0, 0.0, 1.0, 1.0);
    let cyan = Srgba::new(0.0, 1.0, 1.0, 1.0);
    let yellow = Srgba::new(1.0, 1.0, 0.0, 1.0);
    let magenta = Srgba::new(1.0, 0.0, 1.0, 1.0);

    view! (
        linewise_flow [
            inline ColorBox {size: size(6.0, 6.0), color: red},
            inline ColorBox {size: size(21.0, 8.0), color: green},
            MonospaceText (
                ("This is an amazing opportunity to show how cool layouting is!".to_string())
                (white)
            )
            inline ColorBox {size: size(27.0, 2.0), color: magenta},
            inline ColorBox {size: size(12.0, 3.0), color: cyan},
            inline ColorBox {size: size(15.0, 6.0), color: red},
            inline ColorBox {size: size(21.0, 10.0), color: green},
            inline ColorBox {size: size(6.0, 15.0), color: yellow},
            inline ColorBox {size: size(3.0, 5.0), color: blue},
            MonospaceText (
                ("What the hell?".to_string())
                (white)
            )
            inline ColorBox {size: size(3.0, 8.0), color: red},
            inline ColorBox {size: size(9.0, 7.0), color: green},
            inline ColorBox {size: size(15.0, 5.0), color: magenta},
            inline ColorBox {size: size(12.0, 9.0), color: blue},
        ]
    )
}
