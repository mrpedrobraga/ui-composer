#![allow(non_snake_case)]

pub mod views;
pub mod editors;
pub mod containers;

pub use views::*;
pub use editors::*;
pub use containers::*;

#[macro_export]
macro_rules! items_internal {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, crate::items_internal!($($rest)*))
    };
}