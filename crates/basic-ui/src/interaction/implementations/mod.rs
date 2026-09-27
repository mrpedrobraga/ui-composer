use {
    crate::interaction::{Hover, Tap, Typing},
    ui_composer_core::prelude::{Blueprint, Element},
    ui_composer_platform_tui::runner::{TerminalBlueprintResources, TerminalEnvironment},
    ui_composer_state::effect::Effect,
};

impl Blueprint<TerminalEnvironment> for Hover {
    type Output = Self;

    fn make(self, _: &TerminalBlueprintResources) -> Self::Output {
        self
    }
}

impl Element<TerminalEnvironment> for Hover {
    type Effect = ();

    fn effect(&self) -> Self::Effect {}

    type Blueprint = Self;

    fn update(&mut self, blueprint: Self::Blueprint, _: &TerminalBlueprintResources) {
        *self = blueprint
    }
}

impl<A> Blueprint<TerminalEnvironment> for Tap<A>
where
    A: Effect + Send + Sync + 'static,
{
    type Output = Self;

    fn make(self, _: &TerminalBlueprintResources) -> Self::Output {
        self
    }
}

impl<A> Element<TerminalEnvironment> for Tap<A>
where
    A: Effect + Send + Sync + 'static,
{
    type Effect = ();

    fn effect(&self) -> Self::Effect {}

    type Blueprint = Self;

    fn update(&mut self, blueprint: Self::Blueprint, _: &TerminalBlueprintResources) {
        *self = blueprint
    }
}

impl Blueprint<TerminalEnvironment> for Typing {
    type Output = Self;

    fn make(self, _: &TerminalBlueprintResources) -> Self::Output {
        self
    }
}

impl Element<TerminalEnvironment> for Typing {
    type Effect = ();

    fn effect(&self) -> Self::Effect {}

    type Blueprint = Self;

    fn update(&mut self, blueprint: Self::Blueprint, _: &TerminalBlueprintResources) {
        *self = blueprint
    }
}
