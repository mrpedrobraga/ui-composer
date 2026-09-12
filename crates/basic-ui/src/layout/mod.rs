//! # Containers
//!
//! A LayoutItem is kinda like a function, in the sense that it "returns" UI.
//! 
//! Containers are higher-order LayoutItems — they are functions that take
//! LayoutItems as input and return a LayoutItem that uses that input item internally.
//! 
//! For example, a [center][`center::center`] will return a LayoutItem that renders the input item
//! in the middle. This is similar to the parent-child relationship other UI libraries have,
//! but in UI Composer there is no such thing as a UI tree
//! (other than your Rust AST that exists at compile time that is)!
//!
//! ## Sizing
//!
//! Containers follow a sizing law inspired by Algebraic Data Types (ADTs)... Most layout are equivalent to `struct`.
//!
//! Just how `struct` sizes are calculated based on their fields,
//! Containers minimum sizes are defined by their children's.
//! This makes overflow impossible.
//!
//! A container can, however, be _bigger_ than its children.
//! Whenever this happens, the container usually stretches the child
//! to fill all available space.
//! 
//! Some containers, like [center][`center::center`], don't do that,
//! as it's reshaped and stretched, it maintains its item at its own minimum size in the middle of the container.

mod flex;
#[doc(inline)]
pub use flex::*;

pub mod inline;
#[doc(inline)]
pub use inline::*;

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
