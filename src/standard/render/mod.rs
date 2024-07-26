use std::{mem::size_of, thread};

use super::{primitive::Primitive, standard_pipeline::get_main_render_stack_pipeline};
use crate::{
    interaction::{InteractorNode, InteractorNodeContainer, VecNode},
    reaction::Reactor,
    render_module::{IntoRenderModule, RenderModule},
};
use futures_signals::signal::{Mutable, Signal, SignalExt as _};
use tuple_render_module::TupleRenderModule;
use wgpu::{util::DeviceExt as _, BufferUsages};
pub mod tuple_render_module;

pub trait UIFragment: UIFragmentLive + Send {
    fn get_allocation_info() -> AllocationInfo;

    /// Allocates on a render module space for this fragment, but with dummy elements inside.
    /// TODO: Allocate interactors, reactors and submodules.
    fn splat_allocation_empty(
        allocation_offset: AllocationOffset,
        render_module: &mut dyn RenderModule,
        initial: bool,
    ) {
        let info = Self::get_allocation_info();
        if initial {
            for i in 0..info.primitive_count {
                render_module.primitive_buffer().push(Primitive::default())
            }
            for i in 0..info.reactor_count {
                let mut inner_offset = allocation_offset;
                inner_offset.reactor_buffer_offset += 1;
                render_module.reactors().push(None)
            }
            for i in 0..info.interactor_count {
                render_module.interactors().push(None)
            }
        } else {
            for i in 0..info.primitive_count {
                render_module.primitive_buffer()[allocation_offset.primitive_buffer_offset + i] =
                    Primitive::default()
            }
            for i in 0..info.reactor_count {
                render_module.reactors()[allocation_offset.reactor_buffer_offset] = None
            }
            for i in 0..info.interactor_count {
                render_module.interactors()[allocation_offset.interactor_buffer_offset] = None
            }
        }
    }
}

pub trait UIFragmentLive: Send {
    fn splat_allocation(
        &mut self,
        allocation_offset: AllocationOffset,
        render_module: &mut dyn RenderModule,
        initial: bool,
    );
}

#[derive(Clone, Copy, Debug)]
pub struct AllocationInfo {
    pub buffer_size: usize,
    pub primitive_count: usize,
    pub interactor_count: usize,
    pub reactor_count: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct AllocationOffset {
    pub primitive_buffer_offset: usize,
    pub reactor_buffer_offset: usize,
    pub interactor_buffer_offset: usize,
}

impl AllocationOffset {
    pub fn new() -> Self {
        Self {
            primitive_buffer_offset: 0,
            reactor_buffer_offset: 0,
            interactor_buffer_offset: 0,
        }
    }

    pub fn offset_by_allocation(&mut self, allocation: &AllocationInfo) {
        self.primitive_buffer_offset += allocation.primitive_count;
        self.reactor_buffer_offset += allocation.reactor_count;
    }
}

impl<T> IntoRenderModule for T
where
    T: UIFragment,
{
    fn into_render_module<'a, 'window>(
        mut self,
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

        let allocation_info = Self::get_allocation_info();
        let primitive_buffer_cpu = Vec::with_capacity(allocation_info.primitive_count);
        let reactors = Vec::with_capacity(1);

        let mut render_module = TupleRenderModule::new(
            reactors,
            vec![],
            allocation_info.primitive_count as u32,
            primitive_buffer_cpu,
            instance_buffer,
            render_pipeline,
        );

        self.splat_allocation(AllocationOffset::new(), &mut render_module, true);

        render_module.flush_instances(queue);
        return Box::new(render_module);
    }
}
