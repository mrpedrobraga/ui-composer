use crate::standard::runners::wgpu::render_target::Render;
use crate::geometry::layout::LayoutItem;

pub trait UI: LayoutItem<Content: Render + Send> {}
impl<T> UI for T where T: LayoutItem<Content: Render + Send> {}
