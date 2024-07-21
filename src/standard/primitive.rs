use crate::{
    interaction::InteractorNodeContainer,
    prelude::UIFragment,
    render_module::{self, RenderModule},
};
use bytemuck::{Pod, Zeroable};
use std::mem::size_of;

use super::render::{tuple_render_module::TupleRenderModule, AllocationInfo};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Primitive {
    pub transform: [[f32; 4]; 4],
    pub color: [f32; 3],
}

impl UIFragment for Primitive {
    fn get_allocation_info() -> AllocationInfo {
        AllocationInfo {
            buffer_size: size_of::<Self>(),
            primitive_count: 1,
        }
    }

    fn push_allocation(self, render_module: &mut TupleRenderModule) {
        render_module
            .primitive_buffer_cpu
            .extend(bytemuck::cast_slice(&[self]))
    }
}
