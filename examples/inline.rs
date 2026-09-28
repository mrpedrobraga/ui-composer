#![allow(non_snake_case)]
use ::ui_composer_math::palette::rgb::Rgba;
use ::ui_composer_platform_winit::window::Window;
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    UIComposer::run_tui(Terminal(app()))
    // UIComposer::run_winit(Window(app_w()));
}

fn app() -> impl Tui {
    let white = Srgba::new(1.0, 1.0, 1.0, 1.0);
    let red = Srgba::new(1.0, 0.0, 0.0, 1.0);
    let green = Srgba::new(0.0, 1.0, 0.0, 1.0);
    let blue = Srgba::new(0.0, 0.0, 1.0, 1.0);
    let cyan = Srgba::new(0.0, 1.0, 1.0, 1.0);
    let yellow = Srgba::new(1.0, 1.0, 0.0, 1.0);
    let magenta = Srgba::new(1.0, 0.0, 1.0, 1.0);
    let scale = 1.0f32;

    view! (
        linewise_flow [
            inline ColorBox ((scale * size!(6.0, 6.0)) (red))
            inline ColorBox ((size!(21.0, 8.0)) (green))
            MonospaceText (
                ("This is an amazing opportunity to show how cool layouting is!".to_string())
                (white)
            )
            inline ColorBox ((scale * size!(27.0, 2.0)) (magenta))
            inline ColorBox ((scale * size!(12.0, 3.0)) (cyan))
            inline ColorBox ((scale * size!(15.0, 6.0)) (red))
            inline ColorBox ((scale * size!(21.0, 10.0)) (green))
            inline ColorBox ((scale * size!(6.0, 15.0)) (yellow))
            inline ColorBox ((scale * size!(3.0, 5.0)) (blue))
            MonospaceText (
                ("What the hell?".to_string())
                (white)
            )
            inline ColorBox ((scale * size!(3.0, 8.0)) (red))
            inline ColorBox ((scale * size!(9.0, 7.0)) (green))
            inline ColorBox ((scale * size!(15.0, 5.0)) (magenta))
            inline ColorBox ((scale * size!(12.0, 9.0)) (blue))
        ]
    )
}

fn app_w() -> impl WinitUi {
    let white = Srgba::new(1.0, 1.0, 1.0, 1.0);
    let red = Srgba::new(1.0, 0.0, 0.0, 1.0);
    let green = Srgba::new(0.0, 1.0, 0.0, 1.0);
    let blue = Srgba::new(0.0, 0.0, 1.0, 1.0);
    let cyan = Srgba::new(0.0, 1.0, 1.0, 1.0);
    let yellow = Srgba::new(1.0, 1.0, 0.0, 1.0);
    let magenta = Srgba::new(1.0, 0.0, 1.0, 1.0);
    let scale = 8.0f32;

    view! (
        linewise_flow [
            inline ColorBoxW ((scale * size!(6.0, 6.0)) (red))
            inline ColorBoxW ((scale * size!(21.0, 8.0)) (green))
            MonospaceText (
                ("This is an amazing opportunity to show how cool layouting is!".to_string())
                (white)
            )
            inline ColorBoxW ((scale * size!(27.0, 2.0)) (magenta))
            inline ColorBoxW ((scale * size!(12.0, 3.0)) (cyan))
            inline ColorBoxW ((scale * size!(15.0, 6.0)) (red))
            inline ColorBoxW ((scale * size!(21.0, 10.0)) (green))
            inline ColorBoxW ((scale * size!(6.0, 15.0)) (yellow))
            inline ColorBoxW ((scale * size!(3.0, 5.0)) (blue))
            MonospaceText (
                ("What the hell?".to_string())
                (white)
            )
            inline ColorBoxW ((scale * size!(3.0, 8.0)) (red))
            inline ColorBoxW ((scale * size!(9.0, 7.0)) (green))
            inline ColorBoxW ((scale * size!(15.0, 5.0)) (magenta))
            inline ColorBoxW ((scale * size!(12.0, 9.0)) (blue))
        ]
    )
}

#[allow(non_snake_case, unused)]
fn ColorBox(min_size: Size2, color: Rgba) -> impl Tui {
    Canvas::new(move |hx| Graphic {
        rect: hx.rect,
        color,
    })
    .with_minimum_size(min_size)
}

#[allow(non_snake_case, unused)]
fn ColorBoxW(min_size: Size2, color: Rgba) -> impl WinitUi {
    Canvas::new(move |hx| Graphic {
        rect: hx.rect,
        color,
    })
    .with_minimum_size(min_size)
}
