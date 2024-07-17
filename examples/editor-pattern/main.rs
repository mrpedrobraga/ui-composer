#![allow(non_snake_case)]
use futures_signals::signal::Mutable;

fn main() {
    let state = Mutable::new(false);
    let c = Checkbox(state.clone());

    dbg!(state.get());
}

fn Checkbox(state: Mutable<bool>) {
    state.set(true);
}
