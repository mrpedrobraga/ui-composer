pub mod animation;
pub mod process;
pub mod signal_ext;

pub use futures_signals::signal::Mutable;
pub use futures_signals::signal::SignalExt;
pub use futures_signals::signal_map::MutableBTreeMap;
pub use futures_signals::signal_map::SignalMapExt;
pub use futures_signals::signal_vec::MutableVec;
pub use futures_signals::signal_vec::SignalVecExt;

#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a container with interior mutability.",
    label = "This item is not a `Slot`!",
    note = "`Mutable<A>` is a Slot, and can do reactivity on top!"
)]
pub trait Slot {
    type Item;
    fn put(&self, value: Self::Item);
    fn take(&self) -> Self::Item
    where
        Self::Item: Copy;
    fn modify<F>(&self, predicate: F) where F: Fn(&mut Self::Item);
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

    fn modify<F>(&self, mut predicate: F) where F: FnMut(&mut Self::Item) {
        let mut lock = self.lock_mut();
        predicate(&mut *lock);
    }
}

/// Trait that describes an effect — a modification to an environment.
#[must_use = "effects are lazy and do nothing unless applied"]
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
