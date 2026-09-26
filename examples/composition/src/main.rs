use self::{items::Text, runner::Runner};
use ui_composer_view_macro::view;

pub mod element;
pub mod items;
pub mod runner;

fn main() {
    let elements = view! {
        Text::new (("Message one!".to_string()))
    };

    let elements_runner = Runner::new(elements);
    futures::executor::block_on(elements_runner);
}
