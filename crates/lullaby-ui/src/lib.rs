#![allow(non_snake_case)]

pub mod components;

pub mod prelude {
    /* Re-export layout items. */
    pub use ui_composer_basic_ui::layout::*;

    /* Components. */
    pub use crate::components::UI;
    pub use crate::components::button::*;
    pub use crate::components::label::*;
    pub use crate::components::panel_container::*;

    /* Primitives */
    pub use ui_composer_basic_ui::items::*;
    pub use ui_composer_basic_ui::primitives::graphic::Graphic;
    pub use ui_composer_basic_ui::primitives::text::Text;
}

#[macro_export]
macro_rules! list_internal {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, $crate::list_internal!($($rest)*))
    };
}
