use wgpu::BufferUsages;

use super::engine::GPUResources;
use crate::prelude::Quad;

/// The buffers that hold the soon-to-be-rendered UI.
pub struct UINodeRenderBuffers {
    pub instance_buffer_cpu: Vec<Quad>,
    pub instance_buffer: wgpu::Buffer,
}

impl UINodeRenderBuffers {
    /// Creates new buffers for the UI primitives to be drawn.
    pub fn new(gpu_resources: &GPUResources, primitive_count: usize) -> Self {
        Self {
            instance_buffer_cpu: vec![Quad::default(); primitive_count],
            instance_buffer: gpu_resources.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: size_of::<Quad>() as u64 * primitive_count as u64,
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
        }
    }
}
