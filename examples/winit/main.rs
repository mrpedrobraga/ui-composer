#![allow(non_snake_case)]

use chttp::ResponseExt;
use vek::{Rect, Rgba};
use ui_composer::app::composition::effects::future::FutureReactExt;
use ui_composer::app::composition::elements::Blueprint;
use ui_composer::prelude::UIComposer;
use ui_composer::runners::tui::Graphic;
use ui_composer::runners::winit::runner::{WinitEnvironment, WinitRunner};

fn main() {
    #[allow(clippy::unit_arg)]
    UIComposer::run_custom::<WinitRunner<_>>(TestingFuture())
}

fn App() -> impl Blueprint<WinitEnvironment, Element: Send> + Send {
    (
        Graphic {
            rect: Rect::new(0.0, 0.0, 20.0, 20.0),
            color: Rgba::new(0.0, 0.0, 1.0, 1.0),
        },
        Graphic {
            rect: Rect::new(0.0, 0.0, 20.0, 20.0),
            color: Rgba::new(0.0, 0.0, 1.0, 1.0),
        }
    )
}

fn TestingFuture() -> impl Blueprint<WinitEnvironment, Element: Send> + Send {
    let fut = async {
        let mut response = chttp::get_async("https://baconipsum.com/api/?type=meat-and-filler&paras=1&format=text").await.expect("Bacon ipsum failed :-(");
        let text = response.text().expect("Failed to parse response as text.");
        println!("Response: {}", text);
    };

    fut.react()
}