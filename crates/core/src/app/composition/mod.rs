//! #Composition
//! See [`super`] for information and examples of composition.

use crate::app::composition::elements::{Blueprint, Environment};
use layout::LayoutItem;

pub mod algebra;
pub mod effects;
pub mod elements;
pub mod layout;
pub mod visit;

/// Trait for an item that can be used in an app's layout context.
pub trait UI<Env: Environment>:
    LayoutItem<Blueprint: Blueprint<Env, Element: Send>>
{
}
impl<Env: Environment, T> UI<Env> for T where
    T: LayoutItem<Blueprint: Blueprint<Env, Element: Send>>
{
}
