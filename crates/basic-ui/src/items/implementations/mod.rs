use {
    crate::items::{Hover, Tap, Typing},
    ui_composer_core::prelude::{Blueprint, Element},
    ui_composer_platform_tui::runner::TerminalEnvironment,
    ui_composer_state::effect::Effect,
};

impl Blueprint<TerminalEnvironment> for Hover {
    type Element = Self;

    fn make(self, _: &TerminalEnvironment) -> Self::Element {
        self
    }
}

impl Element<TerminalEnvironment> for Hover {
    type Effect<'fx> = ();

    fn effect(&self) -> Self::Effect<'_> {}
}

impl<A> Blueprint<TerminalEnvironment> for Tap<A>
where
    A: Effect + Send + Sync + 'static,
{
    type Element = Self;

    fn make(self, _: &TerminalEnvironment) -> Self::Element {
        self
    }
}

impl<A> Element<TerminalEnvironment> for Tap<A>
where
    A: Effect + Send + Sync + 'static,
{
    type Effect<'fx> = ();

    fn effect(&self) -> Self::Effect<'_> {}
}

impl Blueprint<TerminalEnvironment> for Typing {
    type Element = Self;

    fn make(self, _: &TerminalEnvironment) -> Self::Element {
        self
    }
}

impl Element<TerminalEnvironment> for Typing {
    type Effect<'fx> = ();

    fn effect(&self) -> Self::Effect<'_> {}
}
