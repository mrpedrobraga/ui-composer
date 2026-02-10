use async_std::task::block_on;
use chttp::ResponseExt;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use ui_composer::app::composition::effects::future::FutureExt;
use ui_composer::app::composition::elements::Blueprint;
use ui_composer::app::runner::Runner;
use ui_composer::app::runner::futures::AsyncExecutor;
use ui_composer::prelude::UIComposer;
use ui_composer::state::SignalExt;

/// An environment identifies a platform for which you can develop apps.
pub struct ExampleEnvironment;

/// A runner holds the app and runs it.
pub struct ExampleRunner<B>(PhantomData<B>);

/// The runner must implement the [Runner] trait.
impl<B> Runner for ExampleRunner<B>
where
    B: Blueprint<ExampleEnvironment>,
{
    type AppBlueprint = B;

    /// It has one main function `run` which is responsible for *making* the app
    /// and spawning tasks to:
    ///
    /// 1. Gather and send events;
    /// 2. Poll async elements;
    fn run(ui: Self::AppBlueprint) {
        // Environment is created.
        let env = ExampleEnvironment;

        // App blueprint is *made* into an app.
        let app = ui.make(&env);
        let app = Arc::new(Mutex::new(app));

        println!("[Example] Starting...");

        // We define all tasks to run.
        let tasks = async move {
            // One of them should be an `AsyncExecutor`,
            // which polls the app's futures, streams and signals.
            AsyncExecutor::new(app, env, || {
                println!("[Example] There was an UI update!")
            })
            .to_future()
            .await
        };

        // We block on all tasks.
        block_on(tasks);

        // When all tasks are done, the app will naturally finish.
        // In complex apps, you might use `std::process::exit` instead,
        // but, well, make sure to clean up all your resources!!!
        println!("[Example] Done.")
    }
}

fn main() {
    UIComposer::run_custom::<ExampleRunner<_>>(app())
}

fn app() -> impl Blueprint<ExampleEnvironment, Element: Send> + Send {
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
