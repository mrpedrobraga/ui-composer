use lullaby_ui::prelude::*;
use ui_composer::prelude::*;

fn main() {
    UIComposer::run_tui(Terminal(app()));
}

fn app() -> impl UI {
    view! {
        center
        with_size (size=Extent2::new(40.0, 0.0))
        components {}
    }
}

fn components() -> impl UI {
    view! {
        flex (vertical_flow) [
            /* Views */
            item Label {{"A humble Label..."}}

            /* Effect triggers */
            item Button {
                Label {{"Click this button!"}}
                {|| {}}
            }
        ]
    }
}
