use crate::layout::LayoutItem;
use crate::wgpu::render_target::Render;

pub trait UI: LayoutItem<Content: Render + Send> {}
impl<T> UI for T where T: LayoutItem<Content: Render + Send> {}
