#![allow(non_snake_case)]
pub mod layout;

pub mod text;

#[macro_export]
macro_rules! list_internal {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, $crate::list_internal!($($rest)*))
    };
}
