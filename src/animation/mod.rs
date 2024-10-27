use std::{
    ops::{Add, Mul},
    time::{Duration, Instant},
};

use wgpu::Instance;

pub type Scalar = f32;
pub trait Vector: Copy + Add<Self, Output = Self> + Mul<Scalar, Output = Self> {
    fn linear_interpolate(self, other: Self, factor: Scalar) -> Self {
        (self * (1.0 - factor) + other * factor)
    }
}
impl<T: Copy + Add<Self, Output = Self> + Mul<Scalar, Output = Self>> Vector for T {}

pub struct Animation<TStream: RealTimeStream> {
    initial_value: TStream::Item,
    current_value: TStream::Item,
    stream: TStream,
    start: Instant,
}

impl<TStream: RealTimeStream> Animation<TStream> {
    pub fn new(initial_value: TStream::Item, stream: TStream) -> Self {
        let mut this = Self {
            initial_value,
            current_value: initial_value,
            stream,
            start: Instant::now(),
        };

        this
    }

    pub fn process(&mut self, delta: Duration) -> Poll<TStream::Item> {
        self.stream.process(
            self.initial_value,
            Instant::now().duration_since(self.start),
            delta,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Poll<T> {
    Playing(T),
    Done(T),
}

pub trait RealTimeStream {
    type Item: Copy;

    fn process(
        &mut self,
        initial_value: Self::Item,
        time_elapsed: Duration,
        delta: Duration,
    ) -> Poll<Self::Item>;
}
