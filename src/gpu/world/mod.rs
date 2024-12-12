use std::ops::Deref;

use wgpu::BufferUsages;

use super::engine::GPUResources;
use crate::{prelude::Quad, ui::node::UINode};

/// The buffers that hold the soon-to-be-rendered UI.
pub struct UINodeRenderBuffers {
    pub instance_buffer_cpu: Vec<Quad>,
    pub instance_buffer: wgpu::Buffer,
}

impl UINodeRenderBuffers {
    pub fn get_quad_count(&self) -> usize {
        self.instance_buffer_cpu.len()
    }

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

    pub fn read_quads_from_ui_tree<T>(&mut self, ui_tree: &T)
    where
        T: UINode + ?Sized,
    {
        ui_tree.write_quads(&mut self.instance_buffer_cpu);
    }

    pub fn write_to_gpu(&mut self, gpu_resources: &GPUResources) {
        gpu_resources.queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(self.instance_buffer_cpu.deref()),
        );
    }

    pub fn extend<I>(&mut self, gpu_resources: &GPUResources, new_elements: I)
    where
        I: Iterator<Item = Quad>,
    {
    }
}