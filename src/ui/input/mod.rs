pub mod hover;
pub mod tap;
pub mod window_drag;
use crate::state::Mutable;
pub use hover::*;
pub use tap::*;
pub use window_drag::*;

/// Trait that describes an `Input` element, something that handles user input and mutates state.
pub trait InputItem {}
