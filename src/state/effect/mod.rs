//! # Effect
//!
//! An effect is a change in state.

pub mod animation;

/// Trait that describes an effect — a modification to an environment.
#[must_use = "effects are lazy and do nothing unless applied"]
pub trait Effect: Clone + Send + Sync {
    /// Applies the effect.
    fn apply(&mut self);
}

pub mod implementations {
    use futures_signals::signal::Mutable;
    use crate::state::effect::Effect;

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
}