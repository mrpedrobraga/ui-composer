use ::ui_composer_basic_ui::layout::InlineItem;
use {
    ::ui_composer_core::app::composition::{elements::Environment, CompatibleWith},
    ui_composer_basic_ui::layout::{linewise_flow, MonospaceText},
    ui_composer_math::prelude::Srgba,
};

static TEXT_COLOR: Srgba = Srgba::new(156.0, 78.0, 10.0, 255.0);

pub fn Label<Env: Environment>(string: impl ToString) -> impl CompatibleWith<Env>
where
    MonospaceText: InlineItem<Env>,
{
    linewise_flow(MonospaceText(string.to_string(), TEXT_COLOR / 255.0))
}
