use ::ui_composer_core::app::composition::{elements::Environment, layout::Ui};
use ::ui_composer_math::glamour::Size2;
use crate::layout::{CenterContainer, WithSizeContainer, center, resize};

pub trait UiExt<Env>: Ui<Env> where Env: Environment {
    /// Adapts this item to be centered in its parent.
    /// 
    /// Equivalent to `center(self)`.
    fn centered(self) -> CenterContainer<Env, Self> where Self: Sized {
        center(self)
    }

    /// Adapts this item to have `size` as its minimum size.
    /// 
    /// Equivalent to `with_size(self)`.
    fn with_size(self, size: Size2) -> WithSizeContainer<Env, Self> where Self: Sized {
        resize(self).with_size(size)
    }
}

impl<Env, U> UiExt<Env> for U where U: Ui<Env>, Env: Environment {}