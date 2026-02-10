#![allow(non_snake_case)]

/// Module for optional builtin runners. Might move each runner to a sub-crate.
/// `ui-composer-winit` and `ui-composer-tui` and `ui-composer-embedded`.
pub mod runners;

pub mod components;

#[doc(inline)]
pub use components::layout::*;

#[macro_export]
macro_rules! list_internal {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, $crate::list_internal!($($rest)*))
    };
}
