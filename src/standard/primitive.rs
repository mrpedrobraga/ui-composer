use crate::{
    interaction::InteractorNodeContainer,
    prelude::UIFragment,
    reaction::UnknownReactor,
    render_module::{self, RenderModule},
};
use bytemuck::{Pod, Zeroable};
use std::mem::size_of;

use super::render::{tuple_render_module::TupleRenderModule, AllocationInfo, AllocationOffset};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Primitive {
    pub transform: [[f32; 4]; 4],
    pub color: [f32; 3],
}

impl Default for Primitive {
    fn default() -> Self {
        Primitive {
            transform: Default::default(),
            color: Default::default(),
        }
    }
}

impl UIFragment for Primitive {
    fn get_allocation_info() -> AllocationInfo {
        AllocationInfo {
            buffer_size: size_of::<Self>(),
            primitive_count: 1,
        }
    }

    fn splat_allocation(
        self,
        allocation_offset: AllocationOffset,
        render_module: &mut TupleRenderModule,
        temp_reactors: &mut Vec<Box<dyn UnknownReactor>>,
    ) {
        let offset = allocation_offset.primitive_buffer_offset;

        if render_module.primitive_buffer_cpu.len() == offset {
            render_module.primitive_buffer_cpu.push(self);
        } else {
            render_module.primitive_buffer_cpu[allocation_offset.primitive_buffer_offset] = self;
        }
    }
}
