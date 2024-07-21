use std::mem::size_of;

use super::{primitive::Primitive, standard_pipeline::get_main_render_stack_pipeline};
use crate::{
    interaction::{InteractorNode, InteractorNodeContainer, VecNode},
    render_module::{IntoRenderModule, RenderModule},
};
use tuple_render_module::TupleRenderModule;
use wgpu::{util::DeviceExt as _, BufferUsages};
pub mod tuple_render_module;

pub trait UIFragment {
    fn get_allocation_info() -> AllocationInfo;
    fn push_allocation(self, render_module: &mut TupleRenderModule);
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
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: allocation_info.buffer_size as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut render_module = TupleRenderModule {
            reactors: vec![],
            interactor_tree: Some(Box::new(VecNode::new())),
            sub_modules: vec![],
            primitive_count: allocation_info.primitive_count as u32,
            primitive_buffer_cpu: vec![],
            render_pipeline,
            primitive_buffer: instance_buffer,
        };
        self.push_allocation(&mut render_module);
        render_module.flush_instances(queue);

        return Box::new(render_module);
    }
}
