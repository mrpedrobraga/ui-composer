pub mod effect;
pub mod process;
pub mod signal_ext;

pub use futures_signals::signal::Mutable;
pub use futures_signals::signal::SignalExt;
pub use futures_signals::signal_map::MutableBTreeMap;
pub use futures_signals::signal_map::SignalMapExt;
pub use futures_signals::signal_vec::MutableVec;
pub use futures_signals::signal_vec::SignalVecExt;
pub use futures_signals::signal::Signal;
pub use futures_signals::signal_vec::SignalVec;
use crate::state::effect::Effect;

#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a container with interior mutability.",
    label = "This item is not a `Slot`!",
    note = "`Mutable<A>` is a Slot, and can do reactivity on top!"
)]
/// Trait that describes a slot in which you can read the current value or replace it.
pub trait Slot {
    type Item;
    fn put(&self, value: Self::Item);
    fn take(&self) -> Self::Item
    where
        Self::Item: Copy;
    fn modify<F>(&self, predicate: F)
    where
        F: Fn(&mut Self::Item);
}

/// Trait that describes a state which can change.
pub trait State: Slot {
    /// Returns an effect representing a transformation on this state.
    fn effect<F>(self, predicate: F) -> impl Effect
    where
        Self: Clone + Sized + Send + Sync,
        F: Clone + Send + Sync + Fn(&mut Self::Item),
    {
        move || {
            self.modify(&predicate);
        }
    }
}

pub mod implementations {
    use futures_signals::signal::Mutable;
    use crate::state::{Slot, State};

    impl<A> State for Mutable<A> {}

    impl<A> Slot for Mutable<A> {
        type Item = A;

        fn put(&self, value: Self::Item) {
            self.set(value);
        }

        fn take(&self) -> Self::Item
        where
            Self::Item: Copy,
        {
            self.get()
        }

        fn modify<F>(&self, mut predicate: F)
        where
            F: FnMut(&mut Self::Item),
        {
            let mut lock = self.lock_mut();
            predicate(&mut *lock);
        }
    }
}
