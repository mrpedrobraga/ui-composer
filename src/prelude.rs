pub use crate::app::*;
pub use crate::components::*;
pub use crate::geometry::*;
pub use crate::state::process::{UIFutureExt, UISignalExt};
pub use crate::state::*;
pub use crate::ui::input::*;
pub use crate::ui::layout::*;
pub use futures_signals::signal;
pub use futures_signals::signal_vec;

#[cfg(all(feature = "winit", feature = "wgpu"))]
pub use crate::winitwgpu::window::{Window, WindowAttributes};

#[macro_export]
macro_rules! items {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, ::ui_composer::items!($($rest)*))
    };
}

#[macro_export]
macro_rules! Flex {
    ($( $weight:expr => $item:expr ),* $(,)?) => {
        ::ui_composer::prelude::Flex(
            ::ui_composer::items![
                $(::ui_composer::prelude::FlexItem($item, $weight),)*
            ]
        )
    };
}
