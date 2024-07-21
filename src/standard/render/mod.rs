use super::standard_pipeline::get_main_render_stack_pipeline;
use crate::{
    interaction::{InteractorNode, InteractorNodeContainer, VecNode},
    render_module::{IntoRenderModule, RenderModule},
};
use tuple_render_module::TupleRenderModule;
use wgpu::{util::DeviceExt as _, BufferUsages};
pub mod tuple_render_module;

pub trait UIFragment {
    fn get_allocation_info() -> AllocationInfo;
    fn push_allocation(
        self,
        primitive_buffer: &mut Vec<u8>,
        interactor_node_parent: &mut dyn InteractorNodeContainer,
    );
}

pub struct AllocationInfo {
    pub buffer_size: usize,
    pub primitive_count: usize,
}

impl<T> IntoRenderModule for T
where
    T: UIFragment,
{
    fn into_render_module<'a, 'window>(
        self,
        window: std::sync::Arc<winit::window::Window>,
        surface: wgpu::Surface<'window>,
        adapter: &'a wgpu::Adapter,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
    ) -> Box<dyn RenderModule + 'window> {
        let render_pipeline =
            get_main_render_stack_pipeline(window.clone(), surface, adapter, device, queue);
        let allocation_info = T::get_allocation_info();

        let mut primitive_buffer = vec![];
        let mut root_interactor_node = VecNode::new();

        self.push_allocation(&mut primitive_buffer, &mut root_interactor_node);
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&primitive_buffer[..]),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        let render_module = TupleRenderModule {
            reactors: vec![],
            interactor_tree: Some(Box::new(root_interactor_node)),
            sub_renderers: vec![],
            primitive_count: allocation_info.primitive_count as u32,
            primitive_buffer_cpu: primitive_buffer,
            render_pipeline,
            primitive_buffer: instance_buffer,
        };
        return Box::new(render_module);
    }
}
