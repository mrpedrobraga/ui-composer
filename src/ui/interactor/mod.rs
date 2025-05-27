use super::node::{UIItem};
pub mod hover;
pub mod tap;
pub mod window_drag;
use crate::state::Mutable;
pub use hover::*;
pub use tap::*;
pub use window_drag::*;

/// Trait that describes an `Input` element, something that handles user input and mutates state.
pub trait InputItem {}

/// Trait that describes an effect — a modification to an environment.
pub trait Effect: Clone + Send + Sync {
    /// Applies the effect.
    fn apply(&mut self);
}

impl<F> Effect for F
where
    F: FnMut() + Clone + Send + Sync,
{
    fn apply(&mut self) {
        (self)()
    }
}

impl Effect for Mutable<Option<()>> {
    fn apply(&mut self) {
        self.set(Some(()))
    }
}
