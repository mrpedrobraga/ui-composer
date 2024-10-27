#![allow(non_snake_case)]
use signal::SignalExt;
use ui_composer::{
    components::{Center, WithSize},
    prelude::*,
    ui::react::UISignalExt,
};

#[macro_export]
macro_rules! all {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, all!($($rest)*))
    };
}

pub fn main() {
    App::run(MyApp());
}

pub fn MyApp() -> impl Node {
    Window(Center(WithSize(Extent2::new(32.0, 32.0), Square())))
}

pub fn Square() -> impl LayoutItem {
    Resizable::new(move |hx| {
        let hover = Hover::new(hx.rect);
        let hover_signal = hover.signal();

        all!(
            hover,
            hover_signal
                .map(move |is_hovering| {
                    if is_hovering {
                        Quad::new(hx.rect, Rgb::new(0.4, 0.9, 0.4))
                    } else {
                        Quad::new(hx.rect, Rgb::new(0.9, 0.4, 0.4))
                    }
                })
                .into_ui(),
        )
    })
    .with_minimum_size(Extent2::new(0.0, 0.0))
}
