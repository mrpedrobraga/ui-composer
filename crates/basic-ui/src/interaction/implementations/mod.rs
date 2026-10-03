use ::ui_composer_core::app::composition::elements::Environment;
use {
    crate::interaction::{Hover, Tap, Typing},
    ui_composer_core::prelude::{Blueprint, Element},
    ui_composer_platform_tui::runner::{TerminalBlueprintResources, TerminalEnvironment},
    ui_composer_state::effect::Effect,
};

impl<Env: Environment> Blueprint<Env> for Hover
where
    Hover: Element<Env, Blueprint = Hover>,
{
    type Output = Self;

    fn make(self, _: &Env::BlueprintResources<'_>) -> Self::Output {
        self
    }
}

impl<Env: Environment> Element<Env> for Hover {
    type Effect = ();

    fn effect(&self) -> Self::Effect {}

    type Blueprint = Self;

    fn update(&mut self, blueprint: Self::Blueprint, _: &Env::BlueprintResources<'_>) {
        *self = blueprint
    }
}

impl<Env: Environment, A> Blueprint<Env> for Tap<A>
where
    Tap<A>: Element<Env, Blueprint = Self>,
    A: Effect + Send + Sync + 'static,
{
    type Output = Self;

    fn make(self, _: &Env::BlueprintResources<'_>) -> Self::Output {
        self
    }
}

impl<Env: Environment, A> Element<Env> for Tap<A>
where
    A: Effect + Send + Sync + 'static,
{
    type Effect = ();

    fn effect(&self) -> Self::Effect {}

    type Blueprint = Self;

    fn update(&mut self, blueprint: Self::Blueprint, _: &Env::BlueprintResources<'_>) {
        *self = blueprint
    }
}

impl<Env: Environment> Blueprint<Env> for Typing {
    type Output = Self;

    fn make(self, _: &Env::BlueprintResources<'_>) -> Self::Output {
        self
    }
}

impl<Env: Environment> Element<Env> for Typing {
    type Effect = ();

    fn effect(&self) -> Self::Effect {}

    type Blueprint = Self;

    fn update(&mut self, blueprint: Self::Blueprint, _: &Env::BlueprintResources<'_>) {
        *self = blueprint
    }
}
