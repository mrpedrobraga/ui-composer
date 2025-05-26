use super::node::{UIItem};
pub mod hover;
pub mod tap;
pub mod window_drag;
use crate::state::Mutable;
pub use hover::*;
pub use tap::*;
pub use window_drag::*;

pub trait Interactor {}

pub trait Action {
    // Triggers the action.
    fn trigger(&mut self);
}

impl<F> Action for F
where
    F: FnMut(),
{
    fn trigger(&mut self) {
        (self)()
    }
}

impl Action for Mutable<Option<()>> {
    fn trigger(&mut self) {
        self.set(Some(()))
    }
}
