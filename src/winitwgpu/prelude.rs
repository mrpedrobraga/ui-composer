use crate::layout::LayoutItem;
use crate::wgpu::render_target::RenderDescriptor;

pub trait UI: LayoutItem<Content: RenderDescriptor> {}
impl<T> UI for T where T: LayoutItem<Content: RenderDescriptor> {}
