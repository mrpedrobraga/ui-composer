use ui_composer::app::backend::Runner;
use ui_composer::app::composition::elements::Blueprint;
use ui_composer::standard::prelude::UIComposer;

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

        println!("Boy, I sure do be runnin'!\nApp: {:?}", app);
    }

    async fn event_loop(&self) {
        todo!()
    }

    async fn react_loop(&self) {
        todo!()
    }
}
