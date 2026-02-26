use {
    crate::components::Ui,
    ui_composer_basic_ui::layout::{MonospaceText, linewise_flow},
    ui_composer_math::prelude::Srgba,
};

static TEXT_COLOR: Srgba = Srgba::new(156.0, 78.0, 10.0, 255.0);

pub fn Label(string: impl ToString) -> impl Ui {
    linewise_flow(MonospaceText(string.to_string(), TEXT_COLOR / 255.0))
}
