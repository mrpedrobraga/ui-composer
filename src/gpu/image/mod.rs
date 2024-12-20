use crate::{
    gpu,
    prelude::*,
    ui::{
        self,
        node::{UINode, UINodeDescriptor},
    },
};
use image::{DynamicImage, ExtendedColorType};
use pin_project::pin_project;
use std::io::Read;
use wgpu::BufferUsages;

use super::{
    backend::Node, render_target::GPURenderTarget, texture::ImageRenderTarget,
    world::UINodeRenderBuffers,
};

#[allow(non_snake_case)]
pub fn Image<T>(rect: Rect<f32, f32>, item: T) -> ImageNodeDescriptor<impl UINodeDescriptor>
where
    T: LayoutItem + 'static,
{
    ImageNodeDescriptor {
        rect,
        content: item.bake(LayoutHints { rect }),
    }
}

pub struct ImageNodeDescriptor<T>
where
    T: UINodeDescriptor + 'static,
{
    rect: Rect<f32, f32>,
    content: T,
}

impl<T> NodeDescriptor for ImageNodeDescriptor<T>
where
    T: UINodeDescriptor + 'static,
{
    type ReifiedType = ImageNode;

    fn reify(
        self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        gpu_resources: &gpu::backend::GPUResources,
    ) -> Self::ReifiedType {
        ImageNode {
            rect: self.rect,
            content: Box::new(self.content),
            render_target: ImageRenderTarget::new(gpu_resources, self.rect.extent()),
            content_buffer: UINodeRenderBuffers::new(gpu_resources, T::QUAD_COUNT),
        }
    }
}

#[pin_project(project = ImageNodeProj)]
pub struct ImageNode {
    #[pin]
    content: Box<dyn UINode>,
    rect: Rect<f32, f32>,
    content_buffer: UINodeRenderBuffers,
    render_target: ImageRenderTarget,
}

impl Node for ImageNode {
    fn setup(&mut self, gpu_resources: &gpu::backend::GPUResources) {
        let size_bytes = 4 * 8 * self.rect.w as u64 * self.rect.h as u64;
        let size = self.render_target.image.texture.size();

        self.render_target.draw(
            gpu_resources,
            self.content.as_mut(),
            &mut self.content_buffer,
        );

        let buffer = gpu_resources.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Image Temp Buffer"),
            size: size_bytes,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let buffer = std::sync::Arc::new(buffer);

        let mut encoder =
            gpu_resources
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Image Render Target Encoder"),
                });

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &self.render_target.image.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: buffer.as_ref(),
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * size.width),
                    rows_per_image: Some(size.height),
                },
            },
            size,
        );

        gpu_resources
            .queue
            .submit(std::iter::once(encoder.finish()));

        {
            let buffer_slice = buffer.slice(..);

            let (tx, rx) = std::sync::mpsc::channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            gpu_resources.device.poll(wgpu::Maintain::Wait);
            rx.recv().unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            let v: Vec<_> = data
                .chunks_exact(4)
                .flat_map(|chunk| [chunk[2], chunk[1], chunk[0], chunk[3]])
                .collect();
            use image::{ImageBuffer, Rgba};
            let img_buffer =
                ImageBuffer::<Rgba<u8>, _>::from_raw(size.width, size.height, v).unwrap();
            img_buffer.save("image.png").unwrap();
        }
        buffer.unmap();
    }

    fn handle_window_event(
        &mut self,
        gpu_resources: &gpu::backend::GPUResources,
        window_id: winit::window::WindowId,
        event: ui::node::UIEvent,
    ) {
        self.content.handle_ui_event(event);
    }

    fn poll_processors(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>> {
        let ImageNodeProj { mut content, .. } = self.project();

        let content: &mut _ = &mut **content;
        let content = unsafe { std::pin::Pin::new_unchecked(content) };

        let poll = content.poll_processors(cx);

        match &poll {
            std::task::Poll::Ready(Some(())) => {} // Request redraw
            _ => (),
        }

        poll
    }
}
