use {
    crate::components::Ui,
    ui_composer_basic_ui::layout::{MonospaceText, linewise_flow},
    vek::Rgba,
};

static TEXT_COLOR: Rgba<f32> = Rgba::new(156.0, 78.0, 10.0, 255.0);

pub fn Label(string: impl ToString) -> impl Ui {
    linewise_flow(MonospaceText(string.to_string(), TEXT_COLOR / 255.0))
}
