use ui_composer::prelude::*;

fn main() {
    UIComposer::run(Window(App()));
}

fn App() -> impl LayoutItem {
    Center(Button(Label("Click me..."), || println!("Hello, there!")))
}
