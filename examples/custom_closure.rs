use ui_composer_state::futures_signals::signal::{Mutable, MutableSignal};

fn main() {
    let m = Mutable::new(0);
    let mut c = ReactToSignal(m.signal());

    let _s1 = c.run();
    let _s2 = c.run();
}

trait Closure {
    type Output;

    fn run(&mut self) -> &Self::Output;
}

struct ReactToSignal(MutableSignal<i32>);

impl Closure for ReactToSignal {
    type Output = MutableSignal<i32>;

    fn run(&mut self) -> &Self::Output {
        &self.0
    }
}
