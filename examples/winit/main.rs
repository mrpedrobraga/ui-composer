use ui_composer::runners::winit::runner::WinitRunner;
use ui_composer::standard::prelude::UIComposer;

fn main() {
    #[allow(clippy::unit_arg)]
    UIComposer::run_custom::<WinitRunner<_>>(App())
}

fn App() {}
