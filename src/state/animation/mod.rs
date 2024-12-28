use crate::geometry::Vector;
use futures_time::task;
use futures_time::time::{Duration, Instant};
use std::fmt::Debug;
use std::future::Future;

pub mod spring;

pub trait Slot {
    type Item;
    fn put(&mut self, value: Self::Item);
    fn take(&self) -> Self::Item
    where
        Self::Item: Copy;
}

pub struct Ref<'a, T>(pub &'a mut T);

impl<T> Slot for Ref<'_, T> {
    type Item = T;

    fn put(&mut self, value: Self::Item) {
        *self.0 = value;
    }

    fn take(&self) -> Self::Item
    where
        Self::Item: Copy,
    {
        *self.0
    }
}

/// An alternative of [`Stream`] which is lossy
/// for trying to keep up with an implicit flow of time.
pub trait RealTimeStream {
    type Item;

    /// Processes an animation tick,
    /// indicating, to the stream, that some time passed.
    fn process_tick(
        &mut self,
        initial_value: Self::Item,
        frame_params: AnimationFrameParams,
    ) -> Poll<Self::Item>;

    /// Returns a new stream composed of _this_ stream and [`other`] chained together.
    fn chain<B>(self, other: B) -> Chain<Self, B>
    where
        B: RealTimeStream<Item = Self::Item>,
        Self: Sized,
    {
        Chain {
            stream_a: self,
            stream_b: other,
            stream_a_finished: None,
        }
    }

    /// Returns a new stream that applies a modification on the value.
    fn for_each<F>(self, f: F) -> ForEach<Self, F>
    where
        F: FnMut(&Self::Item) -> (),
        Self: Sized,
    {
        ForEach { inner: self, f }
    }

    fn animate_value<S>(mut self, mut state: S) -> impl Future<Output = ()>
    where
        Self::Item: Copy,
        Self: Sized,
        S: Slot<Item = Self::Item>,
    {
        let initial_value = state.take();
        let start = Instant::now();
        let mut last_frame = Instant::now();

        async move {
            loop {
                let delta = last_frame.elapsed().into();
                let poll = self.process_tick(initial_value, AnimationFrameParams { start, delta });

                match poll {
                    Poll::Ongoing(frame) => {
                        last_frame = Instant::now();
                        state.put(frame);
                    }
                    Poll::Finished(frame) => {
                        state.put(frame);
                        break;
                    }
                }

                task::sleep(Duration::from_millis(16) - last_frame.elapsed().into()).await;
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AnimationFrameParams {
    /// Time since the beginning of the stream.
    pub start: Instant,
    /// Time since the last frame.
    pub delta: Duration,
}

#[derive(Debug)]
pub enum Poll<TItem> {
    Ongoing(TItem),
    Finished(TItem),
}

pub struct Chain<A: RealTimeStream, B: RealTimeStream> {
    stream_a: A,
    stream_b: B,
    stream_a_finished: Option<(A::Item, Instant)>,
}

impl<TItem: Copy, A, B> RealTimeStream for Chain<A, B>
where
    A: RealTimeStream<Item = TItem>,
    B: RealTimeStream<Item = TItem>,
{
    type Item = TItem;

    fn process_tick(
        &mut self,
        initial_value: Self::Item,
        frame_params: AnimationFrameParams,
    ) -> Poll<Self::Item> {
        match &self.stream_a_finished {
            None => {
                let poll = self.stream_a.process_tick(initial_value, frame_params);

                match poll {
                    Poll::Ongoing(frame) => Poll::Ongoing(frame),
                    Poll::Finished(frame) => {
                        let stream_a_end = Instant::now();
                        self.stream_a_finished = Some((frame, stream_a_end));
                        self.stream_b.process_tick(
                            frame,
                            AnimationFrameParams {
                                start: stream_a_end,
                                ..frame_params
                            },
                        )
                    }
                }
            }
            Some((last_a_frame, stream_a_end)) => self.stream_b.process_tick(
                *last_a_frame,
                AnimationFrameParams {
                    start: *stream_a_end,
                    ..frame_params
                },
            ),
        }
    }
}

pub struct ForEach<A: RealTimeStream, F: FnMut(&A::Item) -> ()> {
    inner: A,
    f: F,
}

impl<A, F> RealTimeStream for ForEach<A, F>
where
    A: RealTimeStream,
    F: FnMut(&A::Item) -> (),
{
    type Item = A::Item;

    fn process_tick(
        &mut self,
        initial_value: Self::Item,
        frame_params: AnimationFrameParams,
    ) -> Poll<Self::Item> {
        let mut poll = self.inner.process_tick(initial_value, frame_params);

        match &poll {
            Poll::Ongoing(a) => (self.f)(a),
            Poll::Finished(a) => (self.f)(a),
        }

        poll
    }
}

/// Stream that interpolates the initial value towards a target.
pub struct LinearInterpolateStream<TItem: Vector> {
    to: TItem,
    duration: Duration,
}

impl<TItem: Vector> LinearInterpolateStream<TItem> {
    pub fn new(to: TItem, duration: Duration) -> Self {
        Self { to, duration }
    }
}

impl<TItem: Vector + Copy> RealTimeStream for LinearInterpolateStream<TItem> {
    type Item = TItem;

    fn process_tick(
        &mut self,
        initial_value: Self::Item,
        frame_params: AnimationFrameParams,
    ) -> Poll<Self::Item> {
        if frame_params.start.elapsed() >= self.duration.into() {
            Poll::Finished(self.to)
        } else {
            Poll::Ongoing(initial_value.linear_interpolate(
                self.to,
                frame_params.start.elapsed().as_secs_f32() / self.duration.as_secs_f32(),
            ))
        }
    }
}

pub struct MoveToward<TItem: Vector> {
    current_value: Option<TItem>,
    target: TItem,
    speed: f32,
}
impl<TItem: Vector + Copy> MoveToward<TItem> {
    pub fn new(target: TItem, speed: f32) -> Self {
        MoveToward {
            current_value: None,
            target,
            speed,
        }
    }
}
impl<TItem: Vector + Copy> RealTimeStream for MoveToward<TItem> {
    type Item = TItem;

    fn process_tick(
        &mut self,
        initial_value: Self::Item,
        frame_params: AnimationFrameParams,
    ) -> Poll<Self::Item> {
        if let Some(current_value) = self.current_value {
            let vector = self.target - current_value;
            let next_value = current_value + vector * self.speed * frame_params.delta.as_secs_f32();
            self.current_value = Some(next_value);
            Poll::Ongoing(next_value)
        } else {
            self.current_value = Some(initial_value);
            Poll::Ongoing(initial_value)
        }
    }
}
