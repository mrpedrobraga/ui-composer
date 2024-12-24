#![allow(non_snake_case)]

use futures_time::task;
use futures_time::time::{Duration, Instant};
use ui_composer::items;
use ui_composer::prelude::animation::{AnimationFrameParams, Poll};
use ui_composer::prelude::*;
use ui_composer::state::animation::RealTimeStream;

fn main() {
    App::run(
        Window(Center(Row(
            Row(SmoothSquare("A"), SmoothSquare("B")),
            Row(SmoothSquare("C"), SmoothSquare("D")),
        )))
        .with_title("Interactive Animation"),
    )
}

fn SmoothSquare(name: &'static str) -> impl LayoutItem {
    let is_hovered_state = Editable::new(false);
    let anim_state = Editable::new(0.0);

    Resizable::new(move |hx| {
        let animation = {
            let anim_state = anim_state.clone();
            is_hovered_state.signal().map(move |is_hovering| {
                let spring = Spring::new(if is_hovering { 50.0 } else { 0.0 }, 100.0, 10.0, 1.0);

                spring.animate_state(anim_state.clone()).process()
            })
        };

        let is_hovered_state = is_hovered_state.clone();
        items!(
            animation.process(),
            anim_state
                .signal()
                .map(move |factor| {
                    let rect = hx.rect.translated(-factor * Vec2::unit_y());
                    let hover = Hover::new(rect, is_hovered_state.clone());

                    items!(
                        hover,
                        rect.with_color(Lerp::lerp(Rgb::red(), Rgb::cyan(), factor / 50.0))
                    )
                })
                .process()
        )
    })
    .with_minimum_size(Extent2::new(100.0, 100.0))
}

#[derive(Default)]
struct Spring<T> {
    equilibrium: T,
    stiffness: f32,
    damping: f32,
    mass: f32,
    current_values: Option<(T, T)>,
}

impl<T: Default> Spring<T> {
    fn new(equilibrium: T, stiffness: f32, damping: f32, mass: f32) -> Self {
        Self {
            equilibrium,
            stiffness,
            damping,
            mass,
            ..Default::default()
        }
    }
}

impl RealTimeStream for Spring<f32> {
    type Item = f32;

    fn process_tick(
        &mut self,
        initial_value: Self::Item,
        frame_params: AnimationFrameParams,
    ) -> Poll<Self::Item> {
        let delta_time = frame_params.delta.as_secs_f32();

        // Good ol' Velocity Verlet;
        if let Some((current_value, current_velocity)) = &self.current_values {
            let spring_force = -self.stiffness * (current_value - self.equilibrium);
            let damping_force = -self.damping * current_velocity;
            let total_force = spring_force + damping_force;
            let current_acceleration = total_force / self.mass;

            let next_value = current_value
                + current_velocity * delta_time
                + 0.5 * current_acceleration * delta_time * delta_time;

            let spring_force_new = -self.stiffness * (next_value - self.equilibrium);
            let damping_force_new =
                -self.damping * (current_velocity + current_acceleration * delta_time);
            let total_force_new = spring_force_new + damping_force_new;
            let next_acceleration = total_force_new / self.mass;

            let next_velocity =
                current_velocity + 0.5 * (current_acceleration + next_acceleration) * delta_time;

            self.current_values = Some((next_value, next_velocity));
            Poll::Ongoing(next_value)
        } else {
            self.current_values = Some((initial_value, 0.0));
            Poll::Ongoing(initial_value)
        }
    }
}

impl Spring<f32> {
    async fn animate_state(mut self, state: Editable<f32>) -> () {
        let initial_value = state.get();
        let start = Instant::now();
        let mut last_frame = Instant::now();

        loop {
            let delta = last_frame.elapsed().into();
            let poll = self.process_tick(initial_value, AnimationFrameParams { start, delta });

            match poll {
                Poll::Ongoing(frame) => {
                    last_frame = Instant::now();
                    state.set(frame);
                }
                Poll::Finished(frame) => {
                    state.set(frame);
                    break;
                }
            }

            task::sleep(Duration::from_millis(16) - last_frame.elapsed().into()).await;
        }
    }
}
