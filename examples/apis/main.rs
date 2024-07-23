#![allow(non_snake_case)]
pub use ui_composer::prelude::*;

pub fn main() {
    let using_dark_mode_edt = Editable::new(false);

    let checkbox = Checkbox(using_dark_mode_edt.clone());
}

fn Checkbox(state: Editable<bool>) {
    // Derive over the external state, and possibly mutate it once it receives events.
}
