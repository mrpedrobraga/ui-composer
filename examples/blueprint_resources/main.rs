use ui_composer::app::composition::elements::{Blueprint, DummyEnvironment};

fn main() {
    let blueprint = async_app();

    let env = DummyEnvironment();
    let element = blueprint.make(&env);
}

fn async_app() -> impl Blueprint<DummyEnvironment> {
    
}

