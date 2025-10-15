//! # Containers
//!
//! Containers are higher-order LayoutItems. You can use components to compose primitive [LayoutItem]s,
//! or to alter how they are layed out on your app.
//!
//! ## Sizing
//!
//! Containers follow a sizing law inspired by Algebraic Data Types (ADTs)... Most layout are equivalent to `struct`.
//!
//! Just how `struct` sizes are calculated based on their fields,
//! Containers minimum sizes are defined by their children's. This eliminates overflow.
//!
//! A container can, however, be _bigger_ than its child. Whenever this happens, the container will stretch the child
//! to fill all available space — unless otherwise specified.
//!
//! For example, `Window` makes its layout item fill itself entirely, but [Center] lets the item be small.

mod flex;
#[doc(inline)]
pub use flex::*;

mod center;
#[doc(inline)]
pub use center::*;

mod with_size;
#[doc(inline)]
pub use with_size::*;

mod row;
#[doc(inline)]
pub use row::*;

mod column;
#[doc(inline)]
pub use column::*;
