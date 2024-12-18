pub use crate::app::App;
pub use crate::geometry_ext::*;
pub use crate::gpu::engine::NodeDescriptor;
pub use crate::gpu::window::{Window, WindowAttributes};
pub use crate::signal_ext::*;
pub use crate::ui::graphics::Quad;
pub use crate::ui::interactor::*;
pub use crate::ui::layout::*;
pub use futures_signals::signal;
pub use futures_signals::signal_vec;
pub use vek::*;

#[macro_export]
macro_rules! items {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, item!($($rest)*))
    };
}
