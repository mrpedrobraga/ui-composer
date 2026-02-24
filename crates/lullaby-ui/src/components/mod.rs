use {
    ui_composer_core::app::composition::CompatibleWith,
    ui_composer_platform_tui::runner::TerminalEnvironment,
    //ui_composer_platform_winit::runner::WinitEnvironment,
};

/* Views */
pub mod label;

/* Editors */
pub mod button;

/* Containers */
pub mod panel_container;

pub trait Ui: CompatibleWith<TerminalEnvironment>
//+ CompatibleWith<WinitEnvironment>
{
}

impl<Something> Ui for Something where
    Something: CompatibleWith<TerminalEnvironment> //+ CompatibleWith<WinitEnvironment>
{
}
