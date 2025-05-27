pub mod animation;
pub mod process;
pub mod signal_ext;
use crate::prelude::Effect;
pub use futures_signals::signal::Mutable;
pub use futures_signals::signal::SignalExt;
pub use futures_signals::signal_map::MutableBTreeMap;
pub use futures_signals::signal_map::SignalMapExt;
pub use futures_signals::signal_vec::MutableVec;
pub use futures_signals::signal_vec::SignalVecExt;

pub trait Slot {
    type Item;
    fn put(&self, value: Self::Item);
    fn take(&self) -> Self::Item
    where
        Self::Item: Copy;
}

/// Trait that describes a state which can change.
pub trait State: Slot {
    /// Returns an effect representing a transformation on this state.
    fn effect<F>(self, predicate: F) -> impl Effect
    where
        Self: Clone + Sized + Send + Sync,
        Self::Item: Copy,
        F: Clone + Send + Sync + Fn(Self::Item) -> Self::Item,
    {
        move || {
            let current_value = self.take();
            let new_value = (predicate)(current_value);
            self.put(new_value)
        }
    }
}

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
}
