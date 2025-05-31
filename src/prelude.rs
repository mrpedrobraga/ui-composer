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

// MARK: Geometry and Layout
pub use crate::geometry::*;
pub use crate::layout::*;

pub use crate::components::*;
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
#[allow(non_snake_case)]
macro_rules! Flex {
    ($n:expr ; $( $weight:expr => $item:expr ),* $(,)?) => {
        ::ui_composer::prelude::Flex::<$n, _>(
            ::ui_composer::items![
                $(::ui_composer::prelude::FlexItem($item, $weight),)*
            ]
        )
    };
}

#[macro_export]
macro_rules! or_default {
    (_, $default:expr) => {
        $default
    };
    ($w:expr, $default:expr) => {
        $w
    };
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Flex2 {
    ($n:expr ; $([$weight:tt] $item:expr ),* $(,)?) => {
        ::ui_composer::prelude::Flex::<$n, _>(
            ::ui_composer::items![
                $(
                    ::ui_composer::prelude::FlexItem(
                        $item,
                        ::ui_composer::or_default!($weight, 0.0)
                    ),
                )*
            ]
        )
    };
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! UI {
    (terminal) => {
        impl ::ui_composer::layout::LayoutItem<Content = impl ::ui_composer::tui::pipeline::RenderDescriptor>
    };

    () => {
        impl ::ui_composer::layout::LayoutItem<Content = impl ::ui_composer::wgpu::render_target::Render>
    };
}
