use self::{element::FutureExt as _, items::Text, runner::ElementsRunner};
use futures::future;
use futures_time::{future::FutureExt, time::Duration};

pub mod blueprint;
pub mod element;
pub mod items;
pub mod runner;

fn main() {
    let elements = list![
        Text("MESSAGE ONE!".to_string()),
        delayed(30, 2000).map_element(|value| list![
            Text(format!("Got {value}.")),
            delayed(20, 1000).map_element(move |value2| Text(format!(
                "Got {value2}. Total sum is {}.",
                value + value2
            )))
        ])
    ];

    let elements_runner = ElementsRunner::new(elements);
    futures::executor::block_on(elements_runner);
}

fn delayed<T>(
    value: T,
    millis: u64,
) -> futures_time::future::Delay<future::Ready<T>, futures_time::task::Sleep> {
    future::ready(value).delay(Duration::from_millis(millis))
}
