use crate::app::composition::effects::future::{FutureExt, ReactOnce};
use crate::geometry::Vector;
use crate::state::effect::animation::{Animation, AnimationFrame, Poll};
use core::{
    future::Future,
    ops::Mul,
};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use num_traits::Float;

#[derive(Default)]
/// A [`super::Animation`] that simulates Hooke's law
/// forcing a vector value towards the `equilibrium` point
/// as if it were tied to that point by a string.
pub struct Spring<T> {
    equilibrium: T,
    stiffness: f32,
    damping: f32,
    mass: f32,
    current_values: Option<(T, T)>,
}

impl<T> Spring<T>
where
    T: Default + Vector + Float + Send + Sync + 'static,
    f32: Mul<T, Output = T>,
{
    pub fn new(equilibrium: T, stiffness: f32, damping: f32, mass: f32) -> Self {
        Self {
            equilibrium,
            stiffness,
            damping,
            mass,
            ..Default::default()
        }
    }

    /// Animates `state` between two values depending on whether a condition is met.
    pub fn if_then_else<S, Env>(
        condition: S,
        state: Mutable<T>,
        value_if: T,
        value_else: T,
    ) -> impl Signal<Item = ReactOnce<impl Future<Output = ()>, Env>>
    where
        S: Signal<Item = bool>,
    {
        condition.map(move |is_hovering| {
            let spring = Self::new(
                if is_hovering { value_if } else { value_else },
                800.0,
                20.0,
                0.75,
            );
            spring.animate_value(state.clone()).into_signal()
        })
    }
}


impl<T: Float> Animation for Spring<T>
where
    f32: Mul<T, Output = T>,
    T: Copy + Vector,
{
    type Item = T;

    fn process(
        &mut self,
        initial_value: Self::Item,
        frame_params: AnimationFrame,
    ) -> Poll<Self::Item> {
        let delta_time = frame_params.delta.as_secs_f32();

        // Good ol' Velocity Verlet;
        if let Some((current_value, current_velocity)) = &self.current_values {
            let spring_force = -self.stiffness * (*current_value - self.equilibrium);
            let damping_force = -self.damping * *current_velocity;
            let total_force = spring_force + damping_force;
            let current_acceleration = total_force * (1.0 / self.mass);

            let next_value = *current_value
                + delta_time * (*current_velocity)
                + (0.5f32 * (delta_time * (delta_time * current_acceleration)));

            let spring_force_new = -self.stiffness * (next_value - self.equilibrium);
            let damping_force_new =
                -self.damping * (*current_velocity + delta_time * current_acceleration);
            let total_force_new = spring_force_new + damping_force_new;
            let next_acceleration = total_force_new * (1.0 / self.mass);

            let next_velocity = *current_velocity
                + 0.5f32 * (delta_time * (current_acceleration + next_acceleration));

            self.current_values = Some((next_value, next_velocity));
            Poll::Ongoing(next_value)
        } else {
            self.current_values = Some((initial_value, T::zero()));
            Poll::Ongoing(initial_value)
        }
    }
}
