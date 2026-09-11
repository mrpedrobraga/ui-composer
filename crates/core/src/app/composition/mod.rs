//! #Composition
//! See [`crate::app`] for information and examples of composition.

use crate::app::composition::elements::{Blueprint, Environment};
use layout::LayoutItem;

pub mod algebra;
pub mod effects;
pub mod elements;
pub mod layout;
pub mod visit;

/// This trait marks that an UI item is compatible with a given environment.
/// It's implemented automatically!
pub trait CompatibleWith<Env: Environment>:
    LayoutItem<Blueprint: Blueprint<Env, Element: Send>>
{
}

impl<Env: Environment, T> CompatibleWith<Env> for T where
    T: LayoutItem<Blueprint: Blueprint<Env, Element: Send>>
{
}
