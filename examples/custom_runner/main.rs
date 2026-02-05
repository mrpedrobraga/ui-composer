use async_std::prelude::Stream;
use futures::FutureExt;
use ui_composer::app::runner::Runner;
use ui_composer::app::composition::elements::Blueprint;
use ui_composer::app::input::Event;
use ui_composer::standard::prelude::UIComposer;

fn main() {
    UIComposer::run_custom::<ExampleRunner>(())
}

pub struct ExampleEnvironment;

pub struct ExampleRunner;

impl Runner for ExampleRunner {
    type AppBlueprint = ();

    fn run(ui: Self::AppBlueprint) -> Self {
        let env = ExampleEnvironment;
        #[allow(clippy::let_unit_value)]
        let app = ui.make(&env);

        println!("Boy, I sure do be runnin'!\nApp: {:?}", app);

        Self {}
    }

    fn event_stream(&mut self) -> impl Stream<Item=Event> + 'static {
        async {
            Event::CloseRequested
        }.into_stream()
    }

    fn on_update(&mut self) {
        println!("Something updated!")
    }
}
