#[macro_export]
/// Creates recursive tuples — this is useful because traits can't be implemented
/// for arbitrary-arity tuples.
///
/// Usage:
///
/// ```rust
/// # use ui_composer::items;
/// # let item1 = (); let item2 = (); let item3 = ();
/// items! [
///     item1,
///     item2,
///     item3
/// ]
/// ```
///
/// Produces:
///
/// `(item1, (item2, item3))`
macro_rules! items {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, ::ui_composer::items!($($rest)*))
    };
}

/// Auxiliary macro used internally — it needs to be exported for it to work, you know?
///
/// It takes up to two expressions, returning the left one if it's present,
/// or the right one if the left is absent.
///
/// Usage:
/// ```rust
/// # use ui_composer::or_default;
/// or_default!(3.0, 0.0); //produces '3.0'
/// or_default!(_, 0.0);   //produces '0.0'
/// ```
#[macro_export]
macro_rules! or_default {
    (_, $default:expr) => {
        $default
    };
    ($w:expr, $default:expr) => {
        $w
    };
}

/// Macro for writing flex components without headaches.
///
/// Usage:
/// ```rust
/// # use ui_composer::Flex;
///
/// Flex! { 3;
///     [_] (),
///     [1.0] (),
///     [2.0] ()
/// };
/// ```
///
/// It'd be nice to replace this with a procedural macro that looks like:
///
/// ```compile_fail
/// Flex! {
///     item1,
///     #[grow] item2,
///     #[grow(weight = 2.0)] item3
/// }
/// ```
#[macro_export]
#[allow(non_snake_case)]
macro_rules! Flex {
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