//! #Composition
//! See [`super`] for information and examples of composition.

use crate::app::composition::elements::Blueprint;
use layout::LayoutItem;

pub mod algebra;
pub mod effects;
pub mod elements;
pub mod layout;

/// Trait for an item that can be used in an app's layout context.
pub trait UI<Environment>:
    LayoutItem<Content: Blueprint<Environment, Element: Send>>
{
}
impl<Environment, T> UI<Environment> for T where
    T: LayoutItem<Content: Blueprint<Environment, Element: Send>>
{
}
