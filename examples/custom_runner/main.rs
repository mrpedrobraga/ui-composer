use std::sync::{Arc, Mutex};
use async_std::task::block_on;
use futures_signals::signal::SignalExt;
use ui_composer::app::composition::elements::Blueprint;
use ui_composer::app::runner::futures::AsyncExecutor;
use ui_composer::app::runner::Runner;
use ui_composer::prelude::UIComposer;

fn main() {
    UIComposer::run_custom::<ExampleRunner>(())
}

pub struct ExampleEnvironment;

pub struct ExampleRunner;

impl Runner for ExampleRunner {
    type AppBlueprint = ();

    fn run(ui: Self::AppBlueprint) {
        let env = ExampleEnvironment;
        #[allow(clippy::let_unit_value)]
        let app = ui.make(&env);
        let app = Arc::new(Mutex::new(app));

        println!("[Example] Boy, I sure do be runnin'!\nApp: {:?}", app);
        block_on(AsyncExecutor::new(app, env).to_future());
        println!("[Example] Am no longer.")
    }
}
