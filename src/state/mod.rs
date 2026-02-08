//! # State
//!
//! This module defines the concept of "state" used by apps.
//! In UI Composer apps are immutable, but your app can still "talk" about the concept of state
//! by having those as first-class values.
//!
//! States are values which describe/hold a state. Effects are any change in state.
//! Whereas applications are effect-free, they can still create effects and bubble them up
//! for a [`app::backend::Runner`] to run.
//!
//! States are _very_ useful for building UI since many `impl State<impl UI>: UI`.
//! That is, UI derived from some state will re-render every time that state changes.
//! This drives ALL redrawing and layout in UI Composer.

pub mod effect;
pub mod extensions;

use crate::state::effect::Effect;
pub use futures_signals::signal::Mutable;
pub use futures_signals::signal::Signal;
pub use futures_signals::signal::SignalExt;
pub use futures_signals::signal_map::MutableBTreeMap;
pub use futures_signals::signal_map::SignalMapExt;
pub use futures_signals::signal_vec::MutableVec;
pub use futures_signals::signal_vec::SignalVec;
pub use futures_signals::signal_vec::SignalVecExt;

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
    use crate::state::{Slot, State};
    use futures_signals::signal::Mutable;

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
