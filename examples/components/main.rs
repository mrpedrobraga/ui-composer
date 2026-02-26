use lullaby_ui::prelude::*;
use ui_composer::prelude::*;

fn main() {
    UIComposer::run_tui(Terminal(app()));
}

fn app() -> impl Ui {
    view! {
        center
        components
    }
}

fn components() -> impl Ui {
    let dyn_item = Label("A third label").boxed();

    view! {
        flex {vertical_flow} [
            /* Views */
            item Label (("A humble Label..."))
            item Box::new Label("Another label...")
            item ((dyn_item))

            /* Effect triggers */
            item Button (
                Label (("Click this button!"))
                (|| {})
            )
        ]
    }
}
