#![allow(non_snake_case)]

mod containers;

#[doc(inline)]
pub use containers::*;

#[macro_export]
macro_rules! items_internal {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, $crate::items_internal!($($rest)*))
    };
}
