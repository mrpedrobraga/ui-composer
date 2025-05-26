#![allow(non_snake_case)]

pub mod containers;
pub mod editors;

pub use containers::*;
pub use editors::*;

#[macro_export]
macro_rules! items_internal {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, crate::items_internal!($($rest)*))
    };
}