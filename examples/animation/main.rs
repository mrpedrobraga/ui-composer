use std::thread::spawn;

use anim::{AnimationParams, Interpolate, PlayStatus, RealTimeStream};
use futures_signals::signal::{Mutable, SignalExt};
use futures_time::time::Duration;
use futures_time::{task::sleep, time::Instant};
use pollster::block_on;
use ui_composer::{prelude::*, ui::react::UISignalExt};

pub mod anim;

fn main() {
    App::run(Window(Test()))
}

#[allow(non_snake_case)]
fn Test() -> impl LayoutItem {
    let state = Mutable::new(0.0);
    let state2 = state.clone();
    spawn(move || block_on(animate(state2)));

    Resizable::new(move |hx| {
        state
            .signal()
            .map(move |x| {
                Quad::new(
                    Aabr::new_empty(hx.rect.center() + Vec2::unit_x() * x as f32 * hx.rect.w / 2.0)
                        .into_rect()
                        .expand_radius(10.0),
                    Rgb::blue(),
                )
            })
            .into_ui()
    })
}

async fn animate(state: Mutable<f64>) {
    let lerp: Interpolate<f64> = Interpolate::linear(0.0, 1.0, Duration::from_secs_f64(5.0));
    let dt = Duration::from_millis(16);
    let start = Instant::now();

    loop {
        let frame;
        let mut done = false;
        match lerp.next(AnimationParams::new(start, Instant::now(), dt)) {
            PlayStatus::Playing(f) => frame = f,
            PlayStatus::Done(f) => {
                done = true;
                frame = f
            }
        }

        state.set(frame);
        sleep(dt).await;

        if done {
            break;
        }
    }
}
