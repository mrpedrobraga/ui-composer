//! # Prelude
//!
//! Import this if you want to have common types and macros for ergonomic App creation,
//! while keeping your import list tidy.
//!
//! This isn't, however, the only prelude to import: there are several specialized
//! preludes for each backend. Very interestingly, some backends come with "standard libraries"
//! of components. Between standard libraries, some components will have the same names,
//! like "Button", "Switch", etc.
//!
//! You should be able to switch between libraries with `#[cfg]` attributes
//! on your imports and feature flags.
//!
//! ## Desktop Backend
//!
//! Use `*` from [ui_composer::winitwgpu] to get common types for desktop app creation.
//!
//! ## TUI Backend
//!
//! Use `*` from [ui_composer::tui]

// MARK: General Items

pub use crate::app::{
    backend::Backend,
    input::{items, CursorEvent, EvNum, Event},
    UIComposer,
};

// MARK: Geometry and Layouting
pub use crate::geometry::*;
pub use crate::ui::layout::*;

pub use crate::components::*;
pub use crate::state::process::{UIFutureExt, UISignalExt};
pub use crate::state::*;
pub use futures_signals::signal;
pub use futures_signals::signal_vec;

#[cfg(all(feature = "winit", feature = "wgpu"))]
pub use crate::winitwgpu::window::{Window, WindowAttributes};

#[macro_export]
macro_rules! items {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, ::ui_composer::items!($($rest)*))
    };
}

#[macro_export]
macro_rules! Flex {
    ($( $weight:expr => $item:expr ),* $(,)?) => {
        ::ui_composer::prelude::Flex(
            ::ui_composer::items![
                $(::ui_composer::prelude::FlexItem($item, $weight),)*
            ]
        )
    };
}

#[macro_export]
macro_rules! Component {
    (tui) => {
        impl ::ui_composer::ui::layout::LayoutItem<Content = impl ::ui_composer::tui::pipeline::Render>
    };

    () => {
        impl ::ui_composer::ui::layout::LayoutItem<Content = impl ::ui_composer::winitwgpu::render_target::Render>
    };
}
