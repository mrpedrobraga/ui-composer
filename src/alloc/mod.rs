use std::ops::{Deref, DerefMut};

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferUsages, Device, RenderPass,
};

pub trait IntoGPU {
    fn push_bytes<'slice>(self, buffer: &mut Vec<u8>);
}

pub struct GBox<T: Clone> {
    cpu_obj: T,
    buffer: wgpu::Buffer,
}

impl<T: Clone> GBox<T> {
    pub fn new(device: &Device, cpu_obj: T) -> Self
    where
        T: IntoGPU,
    {
        let mut bytes = Vec::new();
        cpu_obj.clone().push_bytes(&mut bytes);

        Self {
            cpu_obj,
            buffer: device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                usage: BufferUsages::MAP_WRITE,
                contents: bytemuck::cast_slice(bytes.as_slice()),
            }),
        }
    }

    pub fn draw(&self, render_pass: &mut RenderPass) {
        // Here I'll need to send the damn stuff to be drawn
        render_pass.draw(0..3, 0..1);
    }
}

impl<T: Clone> Deref for GBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.cpu_obj
    }
}

impl<T: Clone> DerefMut for GBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cpu_obj
    }
}
