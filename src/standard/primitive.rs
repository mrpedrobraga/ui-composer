use crate::prelude::UIFragment;
use bytemuck::{Pod, Zeroable};
use std::mem::size_of;

use super::render_stack::AllocationInfo;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Primitive {
    pub transform: [[f32; 4]; 4],
    pub color: [f32; 3],
}

impl UIFragment for Primitive {
    fn get_allocation_info(&self) -> AllocationInfo {
        AllocationInfo {
            buffer_size: size_of::<Self>() as u32,
            primitive_count: 1,
        }
    }

    fn push_allocation(&self, primitive_buffer: &mut Vec<u8>) {
        primitive_buffer.extend(bytemuck::cast_slice(&[*self]))
    }
}
