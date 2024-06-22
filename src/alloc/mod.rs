use wgpu::{BufferDescriptor, BufferUsages, Device};

pub struct GPUVec<T> {
    vec: Vec<T>,
    buffer: wgpu::Buffer,
}

impl<T> GPUVec<T> {
    pub fn new(device: &Device) -> Self {
        let vec = Vec::new();
        let buf_capacity = vec.capacity();

        Self {
            vec,
            buffer: device.create_buffer(&BufferDescriptor {
                label: None,
                size: buf_capacity as u64,
                usage: BufferUsages::MAP_WRITE,
                mapped_at_creation: false,
            }),
        }
    }
}
