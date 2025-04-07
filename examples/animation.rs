use futures::join;
use futures_signals::signal::Mutable;
use futures_signals::signal::{Signal, SignalExt};
use futures_time::task;
use futures_time::time::{Duration, Instant};
use std::fmt::Debug;
use std::future::Future;
use std::task::Poll;
use std::thread;
use ui_composer::state::Slot;
use vek::Lerp;

#[pollster::main]
async fn main() {
    let a = Mutable::new(0);
    let b = Mutable::new(10);

    let animation = async move {
        join![
            LerpTo::new(&a, 10, Duration::from_secs(2)).into_future(),
            LerpTo::new(&b, 0, Duration::from_secs(2)).into_future()
        ];

        LerpTo::new(&a, 0, Duration::from_secs(2))
            .into_future()
            .await;
    };

    animation.await;
}

fn notify<S: Signal>(label: &'static str, a: S)
where
    S: Sync + Send + 'static,
    S::Item: Debug,
{
    {
        let notify = a.for_each(move |a_| {
            println!("{} = {:?}", label, a_);
            async {}
        });

        thread::spawn(move || pollster::block_on(notify));
    }
}

trait Animator {
    type Item;

    fn next(&mut self, frame_params: AnimationFrameParams) -> Poll<()>
    where
        Self::Item: Copy;
}

struct AnimatorFuture<A: Animator> {
    animator: A,
    params: AnimationFrameParams,
}

#[derive(Debug, Copy, Clone)]
pub struct AnimationFrameParams {
    /// Time since the beginning of the animation.
    pub start: Instant,
    /// Last frame.
    pub last_frame: Instant,
}

struct LerpTo<'me, S: Slot> {
    slot: &'me S,
    initial_value: Option<S::Item>,
    target: S::Item,
    duration: Duration,
}

impl<'me, S: Slot> LerpTo<'me, S>
where
    S::Item: Copy + Lerp<Output = S::Item>,
{
    fn new(state: &'me S, target: S::Item, duration: Duration) -> Self {
        Self {
            slot: state,
            initial_value: None,
            target,
            duration,
        }
    }
}

impl<'me, S: Slot> LerpTo<'me, S>
where
    S::Item: Copy + Lerp<Output = S::Item>,
{
    fn into_future(mut self) -> impl Future + use<'me, S> {
        let mut params = AnimationFrameParams {
            start: Instant::now(),
            last_frame: Instant::now(),
        };

        async move {
            while let Poll::Pending = self.next(params) {
                task::sleep(Duration::from_millis(16) - params.last_frame.elapsed().into()).await;
                params.last_frame = Instant::now();
            }
        }
    }
}

impl<'me, S: Slot> Animator for LerpTo<'me, S>
where
    S::Item: Copy + Lerp<Output = S::Item>,
{
    type Item = S::Item;

    fn next(&mut self, frame_params: AnimationFrameParams) -> Poll<()> {
        let initial_value = self.initial_value.get_or_insert_with(|| self.slot.take());

        if frame_params.start.elapsed() < self.duration.into() {
            let t = frame_params.start.elapsed().as_secs_f32() / self.duration.as_secs_f32();
            self.slot.put(Lerp::lerp(*initial_value, self.target, t));
            Poll::Pending
        } else {
            self.slot.put(self.target);
            Poll::Ready(())
        }
    }
}
