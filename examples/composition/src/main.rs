use futures::executor::block_on;
use futures_signals::signal::{Mutable, SignalExt};
use futures_time::{task::sleep, time::Duration};

use self::{
    items::{reactive::React, Resizable, Text},
    runner::Runner,
};

pub mod element;
pub mod items;
pub mod runner;

fn main() {
    let state = Mutable::new("Woah");
    let signal = state.signal();

    let state2 = Mutable::new("What?");

    {
        let state2 = state2.clone();
        std::thread::spawn(move || {
            block_on(async move {
                println!("[Start]");
                sleep(Duration::from_secs(1)).await;
                state2.set("What?");
                state.set("Wowzers!");
                sleep(Duration::from_secs(1)).await;
                state.set("Perfect!");
                sleep(Duration::from_secs(1)).await;
                state.set("Gnarly!");
                state2.set("Huh?");
                sleep(Duration::from_secs(1)).await;
                println!("[Change]");
                state2.set("Really?");
                sleep(Duration::from_secs(1)).await;
                state2.set("No cap?");
                state.set("Awesome!");
                state2.set("Nani?");
            })
        });
    }

    let ui = React::new(signal, |afirmation| {
        let signal2 = state2.signal();
        if afirmation.contains("Gna") {
            Some(React::new(signal2, move |question| {
                Resizable::new(move |hx| {
                    Text(format!(
                        "{} - {} - {}",
                        afirmation,
                        question,
                        hx.rect.area()
                    ))
                })
            }))
        } else {
            None
        }
    });

    let runner = Runner::new(ui);
    futures::executor::block_on(runner.to_future());
}
