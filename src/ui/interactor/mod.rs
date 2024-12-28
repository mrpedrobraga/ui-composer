use super::node::{ItemDescriptor, UIItem};
pub mod hover;
pub mod tap;
use crate::state::Mutable;
pub use hover::*;
pub use tap::*;

pub trait Interactor: ItemDescriptor {}

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
