use crate::{interaction::InteractorNodeContainer, prelude::UIFragment};
use bytemuck::{Pod, Zeroable};
use std::mem::size_of;

use super::render::AllocationInfo;

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

    fn push_allocation(self, primitive_buffer: &mut Vec<u8>, _: &mut dyn InteractorNodeContainer) {
        primitive_buffer.extend(bytemuck::cast_slice(&[self]))
    }
}
