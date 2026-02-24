#![allow(non_snake_case)]
use {lullaby_ui::prelude::*, ui_composer::prelude::*};

fn main() {
    UIComposer::run_tui(Terminal(app()))
}

fn app() -> impl TUI {
    view! (
        inline_flow [
            Inline { DebugSquare (size=Extent2::new(6.0, 4.0) color=Rgba::red()) }
            Inline { DebugSquare (size=Extent2::new(21.0, 4.0) color=Rgba::green()) }
            MonospaceText {{
                "This is an amazing opportunity to show how cool layouting is!".to_string()
            }}
            Inline { DebugSquare (size=Extent2::new(27.0, 4.0) color=Rgba::magenta()) }
            Inline { DebugSquare (size=Extent2::new(12.0, 4.0) color=Rgba::blue()) }
            Inline { DebugSquare (size=Extent2::new(15.0, 4.0) color=Rgba::red()) }
            Inline { DebugSquare (size=Extent2::new(21.0, 4.0) color=Rgba::green()) }
            Inline { DebugSquare (size=Extent2::new(6.0, 4.0) color=Rgba::magenta()) }
            Inline { DebugSquare (size=Extent2::new(3.0, 4.0) color=Rgba::blue()) }
            MonospaceText {{ "What the hell?".to_string() }}
            Inline { DebugSquare (size=Extent2::new(3.0, 4.0) color=Rgba::red()) }
            Inline { DebugSquare (size=Extent2::new(9.0, 4.0) color=Rgba::green()) }
            Inline { DebugSquare (size=Extent2::new(15.0, 4.0) color=Rgba::magenta()) }
            Inline { DebugSquare (size=Extent2::new(12.0, 4.0) color=Rgba::blue()) }
        ]
    )
}
