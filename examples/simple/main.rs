#![allow(non_snake_case)]
use std::{
    ops::Mul,
    time::{Duration, Instant},
};
use ui_composer::prelude::*;
use vek::{Lerp, Rect, Rgb};

macro_rules! go {
    ($e:expr) => {
        ::std::thread::spawn(move || ::pollster::block_on(async move { $e }));
    };
}

fn main() {
    AppBuilder::new(App()).run()
}

fn App() -> impl UIFragment {
    let rect1 = Rect::new(100.0, 100.0, 60.0, 60.0);
    let rect2 = Rect::new(600.0, 200.0, 700.0, 700.0);

    let animation_state = Editable::new(1.0);
    let animation_state_proxy = animation_state.clone();
    let start = Instant::now();

    go! {
        loop {
            std::thread::sleep(Duration::from_millis(16));
            animation_state_proxy.set(
                Instant::now()
                    .duration_since(start)
                    .as_secs_f32()
                    .mul(2.0)
                    .sin().mul_add(0.5, 0.5))
        }
    }

    animation_state
        .signal()
        .map(move |lerp_factor| {
            Primitive::rect(
                lerp_rect(rect1, rect2, lerp_factor),
                Lerp::lerp(Rgb::red(), Rgb::green(), lerp_factor),
            )
        })
        .into_fragment()
}

fn lerp_rect<A, B>(from: Rect<A, B>, to: Rect<A, B>, factor: f32) -> Rect<A, B>
where
    A: Lerp<Output = A> + Copy,
    B: Lerp<Output = B> + Copy,
{
    let position = Lerp::lerp(from.position(), to.position(), factor);
    let extent = Lerp::lerp(from.extent(), to.extent(), factor);
    Rect::new(position.x, position.y, extent.w, extent.h)
}
