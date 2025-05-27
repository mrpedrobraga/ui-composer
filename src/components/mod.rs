#![allow(non_snake_case)]

mod containers;
mod views;

#[doc(inline)]
pub use containers::*;
#[doc(inline)]
pub use views::*;

#[macro_export]
macro_rules! items_internal {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, $crate::items_internal!($($rest)*))
    };
}
