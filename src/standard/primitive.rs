use crate::{
    interaction::InteractorNodeContainer,
    prelude::UIFragment,
    reaction::Reactor,
    render_module::{self, RenderModule},
};
use bytemuck::{Pod, Zeroable};
use std::mem::size_of;

use super::render::{
    tuple_render_module::TupleRenderModule, AllocationInfo, AllocationOffset, UIFragmentLive,
};

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
}

impl UIFragmentLive for Primitive {
    fn splat_allocation(
        &mut self,
        allocation_offset: AllocationOffset,
        render_module: &mut dyn RenderModule,
        initial: bool,
    ) {
        let primitive_buffer = render_module.primitive_buffer();
        let offset = allocation_offset.primitive_buffer_offset;

        if initial {
            primitive_buffer.push(*self);
        } else {
            primitive_buffer[allocation_offset.primitive_buffer_offset] = *self;
        }
    }
}
