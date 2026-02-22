use {
    ui_composer_core::app::composition::CompatibleWith,
    ui_composer_platform_tui::runner::TerminalEnvironment,
    //ui_composer_platform_winit::runner::WinitEnvironment,
};

pub mod button;

pub trait UI: CompatibleWith<TerminalEnvironment>
//+ CompatibleWith<WinitEnvironment>
{
}

impl<Something> UI for Something where
    Something: CompatibleWith<TerminalEnvironment> //+ CompatibleWith<WinitEnvironment>
{
}
