#![allow(non_snake_case)]

use chttp::ResponseExt;
use ui_composer::app::composition::effects::future::FutureExt;
use ui_composer::prelude::UIComposer;
use ui_composer::runners::winit::WinitBlueprint;

fn main() {
    UIComposer::run_winit(TestingFuture())
}

fn TestingFuture() -> impl WinitBlueprint {
    let fut = async {
        let text = chttp::get_async(
            "https://baconipsum.com/api/?type=meat-and-filler&paras=1&format=text",
        )
        .await
        .expect("Bacon ipsum failed :-(")
        .text()
        .expect("Failed to parse response as text.");
        println!("Response: {}", text);
    };

    fut.into_signal()
}
