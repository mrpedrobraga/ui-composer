use crate::standard::backends::wgpu::render_target::Render;
use crate::layout::LayoutItem;

pub trait UI: LayoutItem<Content: Render + Send> {}
impl<T> UI for T where T: LayoutItem<Content: Render + Send> {}
