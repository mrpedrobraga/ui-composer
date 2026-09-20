use {
    crate::interaction::{Hover, Tap, Typing},
    ui_composer_core::prelude::{Blueprint, Element},
    ui_composer_platform_tui::runner::{TerminalBlueprintResources, TerminalEnvironment},
    ui_composer_state::effect::Effect,
};

impl Blueprint<TerminalEnvironment> for Hover {
    type Element = Self;

    fn make(self, _: &TerminalBlueprintResources) -> Self::Element {
        self
    }
}

impl Element<TerminalEnvironment> for Hover {
    type Effect = ();

    fn effect(&self) -> Self::Effect {}
}

impl<A> Blueprint<TerminalEnvironment> for Tap<A>
where
    A: Effect + Send + Sync + 'static,
{
    type Element = Self;

    fn make(self, _: &TerminalBlueprintResources) -> Self::Element {
        self
    }
}

impl<A> Element<TerminalEnvironment> for Tap<A>
where
    A: Effect + Send + Sync + 'static,
{
    type Effect = ();

    fn effect(&self) -> Self::Effect {}
}

impl Blueprint<TerminalEnvironment> for Typing {
    type Element = Self;

    fn make(self, _: &TerminalBlueprintResources) -> Self::Element {
        self
    }
}

impl Element<TerminalEnvironment> for Typing {
    type Effect = ();

    fn effect(&self) -> Self::Effect {}
}
