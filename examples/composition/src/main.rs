use futures::FutureExt as _;
use futures_time::{future::FutureExt as _, time::Duration};

use self::{
    items::{Await, Resizable, Text},
    runner::Runner,
};

pub mod element;
pub mod items;
pub mod runner;

fn main() {
    // let ui = Await::new(
    //     async {
    //         println!("Waiting for 3 seconds...");
    //         3
    //     }
    //     .delay(Duration::from_millis(1000)),
    //     |duration_seconds| {
    //         Await::new(
    //             async move { "Ready" }.delay(Duration::from_secs(duration_seconds)),
    //             |text| Resizable::new(move |hx| Text(format!("{text}, {}", hx.rect.area()))),
    //         )
    //     },
    // );

    let long_computation = async { 3 }
        .delay(Duration::from_secs(1))
        .then(|duration| async { "Ready" }.delay(Duration::from_secs(duration)));

    let ui = Await::new(long_computation, |result| {
        Resizable::new(move |hx| Text(format!("{result}, {}", hx.rect.area())))
    });

    let runner = Runner::new(ui);
    futures::executor::block_on(runner);
}
