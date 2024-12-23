use futures_time::time::{Duration, Instant};
use pollster::block_on;
use std::fmt::Debug;
use std::thread::{scope, spawn};
use ui_composer::items;
use ui_composer::prelude::animation::{LinearInterpolateStream, Poll, RealTimeStream};
use ui_composer::prelude::*;
use ui_composer::state::animation::{AnimationFrameParams, MoveToward};
use ui_composer::ui::node::UIItem;
use vek::num_traits::real::Real;

fn main() {
    let state = Editable::new(0.0);

    let a = 0.0;
    let b = 1.0;

    let mut lerp_stream = LinearInterpolateStream::new(b, Duration::from_secs(2))
        .chain(LinearInterpolateStream::new(a, Duration::from_secs(2)))
        .chain(MoveToward::new(b, 2.0));

    {
        let state2 = state.clone();
        spawn(move || {
            block_on(state2.signal().for_each(|frame| {
                println!("{:?}", frame);
                async {}
            }))
        });
    }

    animate(state, &mut lerp_stream);
}

fn animate<T: Copy + Debug, S: RealTimeStream<Item = T>>(state: Editable<T>, stream: &mut S) {
    let initial_value = state.get();
    let start = Instant::now();
    let mut last_frame = Instant::now();

    loop {
        let delta = last_frame.elapsed().into();
        let poll = stream.process_tick(initial_value, AnimationFrameParams { start, delta });

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

        std::thread::sleep(*Duration::from_millis(16) - last_frame.elapsed());
    }
}
