use std::sync::{Arc, LockResult, Mutex};
use crate::app::backend::Runner;
use crate::app::composition::elements::Blueprint;

pub struct WinitEnvironment;

pub type Share<T> = Arc<Mutex<T>>;

pub struct WinitRunner<AppBlueprint> where AppBlueprint : Blueprint<WinitEnvironment> {
    pub app: Share<AppBlueprint::Element>,
}

impl<AppBlueprint> Runner for WinitRunner<AppBlueprint> where AppBlueprint : Blueprint<WinitEnvironment> {
    type AppBlueprint = AppBlueprint::Element;

    fn run(ui: Self::AppBlueprint) {
        todo!()
    }

    async fn event_loop(&self) {
        todo!()
    }

    async fn react_loop(&self) {
        todo!()
    }
}