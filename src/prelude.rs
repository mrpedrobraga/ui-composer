pub use crate::app::App;
pub use crate::components::*;
pub use crate::gpu::backend::Node;
pub use crate::gpu::window::{Window, WindowAttributes};
pub use crate::state::*;
pub use crate::geometry::*;
pub use crate::ui::graphics::Graphic;
pub use crate::ui::interactor::*;
pub use crate::ui::layout::*;
pub use crate::ui::react::UISignalExt;
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
