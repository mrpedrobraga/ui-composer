use ui_composer::app::backend::Runner;
use ui_composer::geometry::layout::ItemBox;
use ui_composer::runners::tui::nodes::TUI;
use ui_composer::runners::tui::{Graphic, TUIRunner, Terminal};
use vek::{Rect, Rgba};

fn main() {
    TUIRunner::run(Terminal(App()))
}

fn App() -> impl TUI {
    ItemBox::new(|_hx| Graphic {
        rect: Rect::new(0.0, 0.0, 3.0, 3.0),
        color: Rgba::cyan(),
    })
}
