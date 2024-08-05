use std::thread::spawn;

use futures_signals::signal::{Mutable, SignalExt};
use futures_time::time::Duration;
use pollster::block_on;
use ui_composer::animation::{animate, Animatable, EaseFn, EasePair, Transition};

fn main() {
    let state = Mutable::new(0);

    let signal = state.signal();
    spawn(|| {
        block_on(signal.for_each(|new_value| async move { println!("New value: {}", new_value) }))
    });

    animate(state.clone())
        .set(10) // !!!
        .slide_to(
            0,
            Transition::WithDuration(
                Duration::from_secs(1),
                EasePair(EaseFn::Linear, EaseFn::Linear),
            ),
        );
}
