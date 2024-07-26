use crate::{
    interaction::InteractorNodeContainer,
    prelude::UIFragment,
    reaction::Reactor,
    render_module::{self, RenderModule},
};
use bytemuck::{Pod, Zeroable};
use std::mem::size_of;
use vek::{Extent3, Mat4, Rect, Rgb};

use super::render::{
    tuple_render_module::TupleRenderModule, AllocationInfo, AllocationOffset, UIFragmentLive,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Primitive {
    pub transform: Mat4<f32>,
    pub color: Rgb<f32>,
}

impl Primitive {
    pub fn rect(rect: Rect<f32, f32>, color: Rgb<f32>) -> Self {
        Self {
            transform: Mat4::identity()
                .scaled_3d(Extent3::new(rect.extent().w, rect.extent().h, 1.0))
                .translated_2d(rect.position()),
            color,
        }
    }
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
            reactor_count: 0,
            interactor_count: 0,
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
