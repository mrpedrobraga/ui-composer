use futures_signals::signal::{Mutable, SignalExt};
use futures_time::time::Duration;
use ui_composer::prelude::animation::{lerp, set, RealTimeStream};
use ui_composer::state::animation::move_toward;
use ui_composer_macros::chain;

fn main() {
    let val = Mutable::new(0.0);

    let anim = chain!({
        yield set(1.0);
        yield lerp(4.0, Duration::from_secs(4));
        yield lerp(0.0, Duration::from_secs(4));
        yield move_toward(4.0, 0.5);
    });

    let sig = val.signal();

    std::thread::spawn(move || {
        futures::executor::block_on(sig.for_each(move |val| {
            println!("{:?}", val);
            async {}
        }))
    });

    futures::executor::block_on(anim.animate_value(val));
}
