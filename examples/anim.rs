use std::fmt::Debug;
use std::time::{Duration, Instant};
use vek::num_traits::real::Real;
use ui_composer::prelude::*;
use ui_composer::prelude::animation::{RealTimeStream, LinearInterpolateStream, Poll};
use ui_composer::state::animation::AnimationFrameParams;

fn main() {
    let a = 0.0;
    let b = 1.0;

    let mut lerp_stream =
        LinearInterpolateStream::new(b, Duration::from_secs(2))
            .chain(LinearInterpolateStream::new(a, Duration::from_secs(2)));

    print_values(a, &mut lerp_stream);
}

fn print_values<T: Copy + Debug, S: RealTimeStream<Item = T>>(initial_value: T, stream: &mut S) {
    let start = Instant::now();
    let mut last_frame = Instant::now();

    loop {
        let delta = last_frame.elapsed();
        let poll = stream.process_tick(initial_value, AnimationFrameParams { start, delta });

        match poll {
            Poll::Ongoing(frame) => {
                last_frame = Instant::now();
                println!("Ongoing... {:?}", frame);
            }
            Poll::Finished(frame) => {
                println!("All done! {:?}", frame);
                break;
            }
        }

        std::thread::sleep(Duration::from_millis(16) - last_frame.elapsed());
    }
}