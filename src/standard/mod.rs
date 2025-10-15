#![allow(non_snake_case)]

mod layout;
/// Module for optional builtin backends. Might move each backend to a sub-crate.
/// `ui-composer-winit` and `ui-composer-tui` and `ui-composer-embedded`.
pub mod backends;
#[doc(hidden)]
#[rust_analyzer::completions(ignore_flyimport)]
pub mod prelude;

#[doc(inline)]
pub use layout::*;

#[macro_export]
macro_rules! items_internal {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, $crate::items_internal!($($rest)*))
    };
}
