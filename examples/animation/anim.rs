use futures_time::time::Duration;
use std::ops::{Add, Mul};
use ui_composer::animation::{AnimationParams, Poll, RealTimeStream};
use vek::Vec2;

pub struct Interpolate<T> {
    start: T,
    end: T,
    duration: Duration,
}
impl<T> Interpolate<T> {
    pub fn linear(start: T, end: T, duration: Duration) -> Self {
        Self {
            start,
            end,
            duration,
        }
    }
}

impl<T> RealTimeStream for Interpolate<T>
where
    T: Copy + Mul<f64, Output = T> + Add<T, Output = T>,
{
    type Item = T;

    fn next(&mut self, animation: AnimationParams) -> Poll<Self::Item> {
        let time_since_start = animation.now.duration_since(*animation.start);
        if time_since_start > *self.duration {
            return Poll::Done(self.end);
        }
        let lerp_factor = time_since_start.as_secs_f64() / self.duration.as_secs_f64();
        return Poll::Playing(self.start * (1.0 - lerp_factor) + self.end * lerp_factor);
    }
}

pub struct Spring {
    target_position: Vec2<f32>,
    dampening: f32,
    item_mass: f32,
    item_position: Vec2<f32>,
    item_velocity: Vec2<f32>,
}

impl Spring {
    pub fn new(
        target_position: Vec2<f32>,
        dampening: f32,
        item_mass: f32,
        start_position: Vec2<f32>,
    ) -> Self {
        Self {
            target_position,
            dampening,
            item_mass,
            item_position: start_position,
            item_velocity: Vec2::zero(),
        }
    }

    pub fn set_position(&mut self, item_position: Vec2<f32>) {
        self.item_position = item_position;
    }

    pub fn set_target_position(&mut self, target_position: Vec2<f32>) {
        self.target_position = target_position;
    }
}

impl RealTimeStream for Spring {
    type Item = Vec2<f32>;

    fn next(&mut self, animation: AnimationParams) -> Poll<Self::Item> {
        let force = self.target_position - self.item_position;
        self.item_velocity += force / self.item_mass;
        self.item_velocity -=
            self.item_velocity * self.dampening * animation.delta_time.as_secs_f32();
        self.item_position += self.item_velocity * animation.delta_time.as_secs_f32();

        Poll::Playing(self.item_position)
    }
}
