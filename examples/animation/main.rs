use std::time::Duration;

use ui_composer::animation::{Animation, Poll, RealTimeStream, Vector};

fn main() {
    let mut anim = Animation::new(1.0, LerpTo::new(5.0, Duration::from_secs(1)));

    loop {
        let frame = anim.process(Duration::from_millis(16));
        match frame {
            Poll::Playing(_) => {
                println!("{:?}", frame);
            }
            Poll::Done(_) => {
                println!("{:?}", frame);
                break;
            }
        }
    }
}

pub struct LerpTo<T: Vector> {
    to: T,
    duration: std::time::Duration,
}

impl<T: Vector> LerpTo<T> {
    fn new(to: T, duration: Duration) -> Self {
        Self { to, duration }
    }
}

impl<T: Vector> RealTimeStream for LerpTo<T> {
    type Item = T;

    fn process(
        &mut self,
        initial_value: Self::Item,
        time_elapsed: std::time::Duration,
        delta: std::time::Duration,
    ) -> ui_composer::animation::Poll<Self::Item> {
        if time_elapsed >= self.duration {
            Poll::Done(self.to)
        } else {
            Poll::Playing(initial_value.linear_interpolate(
                self.to,
                time_elapsed.as_secs_f32() / self.duration.as_secs_f32(),
            ))
        }
    }
}
