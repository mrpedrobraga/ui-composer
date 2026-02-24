#![allow(non_snake_case)]

pub mod components;
pub mod convert;

pub use ui_composer_basic_ui::components::image::image;
pub mod prelude {
    /* Re-export layout items. */
    pub use ui_composer_basic_ui::components::*;
    pub use ui_composer_basic_ui::layout::*;

    /* Primitives */
    pub use ui_composer_basic_ui::interaction::*;
    pub use ui_composer_basic_ui::primitives::graphic::Graphic;
    pub use ui_composer_basic_ui::primitives::image_quad::ImageView;
    pub use ui_composer_basic_ui::primitives::text::Text;

    /* Components. */
    pub use crate::components::Ui;
    pub use crate::components::button::*;
    pub use crate::components::label::*;
    pub use crate::components::panel_container::*;

    /* Traits */
    pub use crate::convert::ToDefaultUi;
}

#[macro_export]
macro_rules! list_internal {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, $crate::list_internal!($($rest)*))
    };
}
