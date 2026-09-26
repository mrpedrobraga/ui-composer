use self::{items::Text, runner::ElementsRunner};

pub mod element;
pub mod items;
pub mod runner;

fn main() {
    let elements = list![Text::new("MESSAGE ONE!".to_string()),];

    let elements_runner = ElementsRunner::new(elements);
    futures::executor::block_on(elements_runner);
}
