//! # UI Composer Core
//!
//! > [!NOTE]
//! > To learn about UI Composer as a library, see the workspace readme.
//!
//! This crate contains the basic concepts for the "composition"-based architecture
//! UI Composer makes use of everywhere.
//!
//! To start, check the documentation for [`app`].

pub mod app;

pub mod prelude {
    pub use crate::app::composition::algebra::{Bubble, Empty, Gather, Monoid, Semigroup};
    pub use crate::app::composition::effects::signal::SignalExt;
    pub use crate::app::composition::elements::{Blueprint, Element, Environment};
    pub use crate::app::composition::layout::{Canvas, Resizable, Ui};
    pub use crate::app::composition::visit::{Apply, ApplyMut, DriveThru, DriveThruMut};
    pub use crate::app::runner::futures::AsyncExecutor;
    pub use crate::app::runner::Runner;
}
