use std::future::Future;

use futures_signals::signal::Mutable;
use futures_time::{future::FutureExt as _, time::Duration};

/// Animates a state as a process.
pub fn animate<T>(state: Mutable<T>) -> AnimatableMutable<T> {
    AnimatableMutable { state }
}

/// Trait defines a state which can be animated.
pub trait Animatable {
    type Item;

    /// Sets the internal state to this new value.
    fn set_state(&self, new_value: Self::Item);

    /// Animates the internal state...
    fn animate(&self) -> impl Future<Output = ()>;

    /// Immediately sets the value of the state.
    fn set(self, value: Self::Item) -> Set<Self>
    where
        Self: Sized,
    {
        Set {
            animation: self,
            value,
        }
    }

    /// Waits [`duration`] seconds.
    fn wait(self, duration: Duration) -> Wait<Self>
    where
        Self: Sized,
    {
        Wait {
            animation: self,
            duration,
        }
    }

    fn slide_to(self, target_value: Self::Item, transition: Transition) -> SlideTo<Self>
    where
        Self: Sized,
    {
        SlideTo {
            animation: self,
            target_value,
            transition,
        }
    }
}

pub struct AnimatableMutable<T> {
    state: Mutable<T>,
}
impl<T> Animatable for AnimatableMutable<T> {
    type Item = T;

    fn set_state(&self, new_value: Self::Item) {
        self.state.set(new_value);
    }

    fn animate(&self) -> impl Future<Output = ()> {
        async {}
    }
}

pub struct Set<A: Animatable> {
    animation: A,
    value: A::Item,
}
impl<A: Animatable<Item = T>, T: Copy> Animatable for Set<A> {
    type Item = A::Item;

    fn set_state(&self, new_value: Self::Item) {
        self.animation.set_state(new_value)
    }

    fn animate(&self) -> impl Future<Output = ()> {
        self.set_state(self.value);
        async {}
    }
}

pub struct Wait<A: Animatable> {
    animation: A,
    duration: Duration,
}
impl<A: Animatable> Animatable for Wait<A> {
    type Item = A::Item;

    fn set_state(&self, new_value: Self::Item) {
        self.animation.set_state(new_value)
    }

    fn animate(&self) -> impl Future<Output = ()> {
        self.animation.animate().delay(self.duration)
    }
}

pub struct SlideTo<A: Animatable> {
    animation: A,
    target_value: A::Item,
    transition: Transition,
}

pub enum Transition {
    /// Sets the new value immediately
    Immediate,
    /// Slides the value over time, taking
    WithDuration(Duration, EasePair),
    WithVelocity,
    WithForce,
}

pub struct EasePair(pub EaseFn, pub EaseFn);

pub enum EaseFn {
    Linear,
    Quadratic,
    Cubic,
    Quartic,
    Quintic,
    Sine,
    Circular,
    Exponential,
    Bounce,
    Elastic,
}
