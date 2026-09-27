use futures::executor::block_on;
use futures_signals::signal::{Mutable, SignalExt};
use futures_time::{task::sleep, time::Duration};

use self::{
    items::{reactive::SignalExt as _, Resizable, Text},
    runner::Runner,
};

pub mod element;
pub mod items;
pub mod runner;

macro_rules! zip {
    ($($id:ident),*) => {
        ::futures_signals::map_ref! { $($id),* => ( $(*$id),* ) }
    };
}

fn main() {
    let state = Mutable::new("Woah");
    let affirmation = state.signal();

    let state2 = Mutable::new("What?");
    let question = state2.signal();

    {
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

    // let combined_signal = map_ref! {affirmation, question => (*affirmation, *question)};
    let combined_signal = zip!(affirmation, question);

    let ui = combined_signal.react(|(affirmation, question)| {
        if_then(
            affirmation.contains("Gna"),
            Some(Resizable::new(move |hx| {
                Text(format!(
                    "{} - {} - {}",
                    affirmation,
                    question,
                    hx.rect.area()
                ))
            })),
        )
    });

    let runner = Runner::new(ui);
    futures::executor::block_on(runner.to_future());
}

fn if_then<U>(condition: bool, ui: U) -> Option<U> {
    if condition {
        Some(ui)
    } else {
        None
    }
}
