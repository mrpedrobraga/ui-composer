//! # Animation
//!
//! An animation is a coordinated, continuous and long-lived effect.
//! Animation is to Effect what Stream is to Future.

use crate::geometry::Vector;
use crate::{geometry::Lerp, state::Slot};
use core::{fmt::Debug, future::Future};
use futures_time::{
    task,
    time::{Duration, Instant},
};

pub mod spring;

/// A lossy [`Stream`] which attempts to keep up with the flow of time.
pub trait Animation {
    type Item;

    /// Processes an animation tick,
    /// indicating, to the stream, that some time passed.
    fn process(
        &mut self,
        initial_value: Self::Item,
        frame_params: AnimationFrame,
    ) -> Poll<Self::Item>
    where
        Self::Item: Copy;

    /// Returns a new stream composed of _this_ stream and [`other`] chained together.
    fn chain<B>(self, other: B) -> Chain<Self, B>
    where
        B: Animation<Item = Self::Item>,
        Self: Sized,
    {
        Chain {
            stream_a: self,
            stream_b: other,
            stream_a_finished: None,
        }
    }

    /// Chains [self] with an additional linear interpolation of the value.
    fn lerp_to(
        self,
        target_value: Self::Item,
        duration: Duration,
    ) -> Chain<Self, LinearInterpolate<Self::Item>>
    where
        Self: Sized,
        Self::Item: Lerp,
    {
        self.chain(lerp(target_value, duration))
    }

    /// Returns a new stream that applies a modification on the value.
    fn for_each<F>(self, f: F) -> ForEach<Self, F>
    where
        F: FnMut(&Self::Item),
        Self: Sized,
    {
        ForEach { inner: self, f }
    }

    /// Consumes this [Animation] and produces a future that completes
    /// when the animation is finished.
    fn animate_from<F>(
        mut self,
        mut f: F,
        initial_value: Self::Item,
    ) -> impl Future<Output = ()>
    where
        Self::Item: Copy,
        Self: Sized,
        F: FnMut(Self::Item),
    {
        let start = Instant::now();
        let mut last_frame = Instant::now();
        async move {
            loop {
                let delta = last_frame.elapsed().into();
                let poll = self
                    .process(initial_value, AnimationFrame { start, delta });

                match poll {
                    Poll::Ongoing(frame) => {
                        last_frame = Instant::now();
                        f(frame);
                    }
                    Poll::Finished(frame) => {
                        f(frame);
                        break;
                    }
                }

                task::sleep(
                    Duration::from_millis(16) - last_frame.elapsed().into(),
                )
                .await;
            }
        }
    }

    /// Equivalent to [std::iter::Iterator::collect], it consumes the animation
    /// to animate a value through a slot.
    fn animate_value<S>(self, state: S) -> impl Future<Output = ()>
    where
        Self::Item: Copy,
        Self: Sized,
        S: Slot<Item = Self::Item> + 'static,
    {
        let initial_value = state.take();
        self.animate_from(move |frame| state.put(frame), initial_value)
    }

    /// Same as [Self::animate_value] but animates with a slot reference...
    fn animate_ref<S>(self, state: &S) -> impl Future<Output = ()>
    where
        Self::Item: Copy,
        Self: Sized,
        S: Slot<Item = Self::Item> + 'static,
    {
        let initial_value = state.take();
        self.animate_from(move |frame| state.put(frame), initial_value)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AnimationFrame {
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

pub struct Chain<A: Animation, B: Animation> {
    stream_a: A,
    stream_b: B,
    stream_a_finished: Option<(A::Item, Instant)>,
}

impl<TItem: Copy, A, B> Animation for Chain<A, B>
where
    A: Animation<Item = TItem>,
    B: Animation<Item = TItem>,
{
    type Item = TItem;

    fn process(
        &mut self,
        initial_value: Self::Item,
        frame_params: AnimationFrame,
    ) -> Poll<Self::Item> {
        match &self.stream_a_finished {
            None => {
                let poll = self.stream_a.process(initial_value, frame_params);

                match poll {
                    Poll::Ongoing(frame) => Poll::Ongoing(frame),
                    Poll::Finished(frame) => {
                        let stream_a_end = Instant::now();
                        self.stream_a_finished = Some((frame, stream_a_end));
                        self.stream_b.process(
                            frame,
                            AnimationFrame {
                                start: stream_a_end,
                                ..frame_params
                            },
                        )
                    }
                }
            }
            Some((last_a_frame, stream_a_end)) => self.stream_b.process(
                *last_a_frame,
                AnimationFrame {
                    start: *stream_a_end,
                    ..frame_params
                },
            ),
        }
    }
}

pub struct ForEach<A: Animation, F: FnMut(&A::Item)> {
    inner: A,
    f: F,
}

impl<A, F> Animation for ForEach<A, F>
where
    A: Animation,
    F: FnMut(&A::Item),
{
    type Item = A::Item;

    fn process(
        &mut self,
        initial_value: Self::Item,
        frame_params: AnimationFrame,
    ) -> Poll<Self::Item>
    where
        Self::Item: Copy,
    {
        let poll = self.inner.process(initial_value, frame_params);

        match &poll {
            Poll::Ongoing(a) => (self.f)(a),
            Poll::Finished(a) => (self.f)(a),
        }

        poll
    }
}

/// Assigns the animated value to this value immediately.
pub fn assign<Item>(value: Item) -> Assign<Item> {
    Assign(value)
}

/// See [assign].
pub struct Assign<Item>(Item);

impl<Item> Animation for Assign<Item> {
    type Item = Item;

    fn process(
        &mut self,
        _initial_value: Self::Item,
        _frame_params: AnimationFrame,
    ) -> Poll<Self::Item>
    where
        Self::Item: Copy,
    {
        Poll::Finished(self.0)
    }
}

/// Interpolates the initial value to a destination value in a certain time.
pub fn lerp<Item: Lerp>(
    to: Item,
    duration: Duration,
) -> LinearInterpolate<Item> {
    LinearInterpolate { to, duration }
}

/// See [lerp].
pub struct LinearInterpolate<Item: Lerp> {
    to: Item,
    duration: Duration,
}

impl<Item: Lerp> Animation for LinearInterpolate<Item> {
    type Item = Item;

    fn process(
        &mut self,
        initial_value: Self::Item,
        frame_params: AnimationFrame,
    ) -> Poll<Self::Item>
    where
        Self::Item: Copy,
    {
        if frame_params.start.elapsed() >= self.duration.into() {
            Poll::Finished(self.to)
        } else {
            Poll::Ongoing(initial_value.linear_interpolate(
                self.to,
                frame_params.start.elapsed().as_secs_f32()
                    / self.duration.as_secs_f32(),
            ))
        }
    }
}

/// Moves towards the target value with a scalar speed.
/// Unlike [lerp], this animation does not have a defined duration,
/// instead, it takes longer the further away the value is from the target.
pub fn move_toward<Item: Lerp>(target: Item, speed: Item) -> MoveToward<Item> {
    MoveToward {
        current_value: None,
        target,
        speed,
    }
}

/// See [move_toward].
pub struct MoveToward<Item: Lerp> {
    current_value: Option<Item>,
    target: Item,
    speed: Item,
}

impl<Item> Animation for MoveToward<Item>
where
    Item: Lerp + Vector + Copy,
{
    type Item = Item;

    fn process(
        &mut self,
        initial_value: Self::Item,
        frame_params: AnimationFrame,
    ) -> Poll<Self::Item> {
        if let Some(current_value) = self.current_value {
            let vector = self.target - current_value;
            let next_value = current_value
                + vector * self.speed * frame_params.delta.as_secs_f32();
            self.current_value = Some(next_value);
            Poll::Ongoing(next_value)
        } else {
            self.current_value = Some(initial_value);
            Poll::Ongoing(initial_value)
        }
    }
}
