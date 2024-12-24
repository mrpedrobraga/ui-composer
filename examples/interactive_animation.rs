#![allow(non_snake_case)]

use futures_signals::signal::Signal;
use futures_time::task;
use futures_time::time::{Duration, Instant};
use ui_composer::items;
use ui_composer::prelude::animation::{AnimationFrameParams, Poll};
use ui_composer::prelude::*;
use ui_composer::state::animation::RealTimeStream;

fn main() {
    App::run(
        Window(Center(Row(
            Row(
                SmoothSquare(Rgb::new(214.0 / 255.0, 93.0 / 255.0, 177.0 / 255.0)),
                SmoothSquare(Rgb::new(255.0 / 255.0, 111.0 / 255.0, 145.0 / 255.0)),
            ),
            Row(
                SmoothSquare(Rgb::new(255.0 / 255.0, 150.0 / 255.0, 113.0 / 255.0)),
                SmoothSquare(Rgb::new(255.0 / 255.0, 199.0 / 255.0, 95.0 / 255.0)),
            ),
        )))
        .with_title("Interactive Animation"),
    )
}

fn SmoothSquare(color: Rgb<f32>) -> impl LayoutItem {
    let is_hovered_state = Editable::new(false);
    let anim_state = Editable::new(0.0);

    Resizable::new(move |hx| {
        let is_hovered_state = is_hovered_state.clone();
        items!(
            if_then_else(is_hovered_state.signal(), anim_state.clone(), 50.0, 0.0).process(),
            anim_state
                .signal()
                .map(move |animation_factor| {
                    hover_square(hx.rect, color, animation_factor, is_hovered_state.clone())
                })
                .process()
        )
    })
    .with_minimum_size(Extent2::new(100.0, 100.0))
}

fn hover_square(
    original_rect: Rect<f32, f32>,
    original_color: Rgb<f32>,
    animation_factor: f32,
    is_hovered_state: Editable<bool>,
) -> impl ItemDescriptor {
    let hover = Hover::new(
        original_rect.expanded_to_contain_point(Vec2::new(
            original_rect.x,
            original_rect.y - animation_factor,
        )),
        is_hovered_state,
    );
    let rect = original_rect
        .translated(-animation_factor * Vec2::unit_y())
        .expand(8.0 * animation_factor / 50.0);

    items!(
        hover,
        rect.with_color(Lerp::lerp(
            0.75 * original_color,
            original_color,
            animation_factor / 50.0
        ))
    )
}

fn if_then_else<S>(
    condition: S,
    state: Editable<f32>,
    value_if: f32,
    value_else: f32,
) -> impl Signal<Item = impl ItemDescriptor>
where
    S: Signal<Item = bool>,
{
    condition.map(move |is_hovering| {
        let spring = Spring::new(
            if is_hovering { value_if } else { value_else },
            200.0,
            10.0,
            1.0,
        );
        spring.animate_state(state.clone()).process()
    })
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
