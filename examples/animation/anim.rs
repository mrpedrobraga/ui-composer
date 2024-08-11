use futures_time::time::{Duration, Instant};
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimationParams {
    /// The moment the animation started.
    pub start: Instant,
    /// The present instant
    pub now: Instant,
    /// Time elapsed since the previous frame.
    pub delta_time: Duration,
}

impl AnimationParams {
    pub fn new(start: Instant, now: Instant, delta_time: Duration) -> Self {
        Self {
            start,
            now,
            delta_time,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayStatus<T> {
    Playing(T),
    Done(T),
}

/// A Stream that can be consumed in real time by passing in the elapsed time since the previous frame.
pub trait RealTimeStream {
    /// The item this stream yields.
    type Item;

    fn next(&self, animation: AnimationParams) -> PlayStatus<Self::Item>;
}

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

    fn next(&self, animation: AnimationParams) -> PlayStatus<Self::Item> {
        let time_since_start = animation.now.duration_since(*animation.start);
        if time_since_start > *self.duration {
            return PlayStatus::Done(self.end);
        }
        let lerp_factor = time_since_start.as_secs_f64() / self.duration.as_secs_f64();
        return PlayStatus::Playing(self.start * (1.0 - lerp_factor) + self.end * lerp_factor);
    }
}
