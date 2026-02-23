/// Creates recursive tuples — this is useful because traits can't be implemented
/// for arbitrary-arity tuples.
///
/// Usage:
///
/// ```rust
/// # use ui_composer::list;
/// # let item1 = (); let item2 = (); let item3 = ();
/// list! [
///     item1,
///     item2,
///     item3
/// ]
/// ```
///
/// Produces:
///
/// `(item1, (item2, item3))`
#[doc(hidden)]
#[macro_export]
macro_rules! __list {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, $crate::__list!($($rest)*))
    };
}

#[doc(inline)]
pub use __list as list;
