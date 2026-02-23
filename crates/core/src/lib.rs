pub mod app;

pub mod prelude {
    pub use crate::app::composition::algebra::{
        Bubble, Empty, Gather, Monoid, Semigroup,
    };
    pub use crate::app::composition::effects::{
        future::IntoBlueprint as _, signal::IntoBlueprint as _,
    };
    pub use crate::app::composition::elements::{
        Blueprint, Element, Environment,
    };
    pub use crate::app::composition::layout::{ItemBox, LayoutItem, Resizable};
    pub use crate::app::composition::visit::{
        Apply, ApplyMut, DriveThru, DriveThruMut,
    };
    pub use crate::app::runner::Runner;
    pub use crate::app::runner::futures::AsyncExecutor;
}
