//! #Composition
//! See [`super`] for information and examples of composition.

use crate::app::composition::elements::Blueprint;
use crate::geometry::layout::LayoutItem;

pub mod algebra;
#[deprecated]
pub mod reify;
pub mod implementations;
pub mod elements;
pub mod effects;

/// Trait for an item that can be used in an app's layout context.
pub trait UI<Environment>: LayoutItem<Content: Blueprint<Environment, Element: Send>> {}
impl<Environment, T> UI<Environment> for T where T: LayoutItem<Content: Blueprint<Environment, Element: Send>> {}