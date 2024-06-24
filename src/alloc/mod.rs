use std::ops::{Deref, DerefMut};

use wgpu::{BufferDescriptor, BufferUsages, Device};

pub struct GBox<T> {
    cpu_obj: T,
    buffer: wgpu::Buffer,
}

impl<T> GBox<T> {
    pub fn new(device: &Device, cpu_obj: T) -> Self {
        let buf_capacity = std::mem::size_of::<T>();

        Self {
            cpu_obj,
            buffer: device.create_buffer(&BufferDescriptor {
                label: None,
                size: buf_capacity as u64,
                usage: BufferUsages::MAP_WRITE,
                mapped_at_creation: false,
            }),
        }
    }
}

impl<T> Deref for GBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.cpu_obj
    }
}

impl<T> DerefMut for GBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cpu_obj
    }
}
