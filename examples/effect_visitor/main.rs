use async_std::task::block_on;
use chttp::ResponseExt;
use futures_signals::signal::SignalExt;
use ui_composer::app::composition::elements::{Blueprint, DummyEnvironment};
use ui_composer::app::composition::effects::executor::DummyExecutor;
use ui_composer::app::composition::effects::future::FutureReactExt;

pub mod render;

#[macro_export]
macro_rules! tuple {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, $crate::tuple!($($rest)*))
    };
}

fn main() {
    let blueprint = async_app();

    let env = DummyEnvironment();
    let element = blueprint.make(&env);
    let e = DummyExecutor { element };

    block_on(e.to_future());
}

fn async_app() -> impl Blueprint<DummyEnvironment> {
    let block = async {
        let url = "https://baconipsum.com/api/?type=all-meat&sentences=1&format=text";
        let result = chttp::get_async(url)
            .await
            .unwrap()
            .text_async()
            .await
            .unwrap();
        println!("{result}");
    };

    block.react()
}
