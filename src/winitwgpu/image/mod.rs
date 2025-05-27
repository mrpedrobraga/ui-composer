use {
    super::{
        backend::ReifiedNode,
        pipeline::{graphics::RenderGraphicDescriptor, text::TextPipelineBuffers},
        render_target::Render,
        texture::ImageRenderTarget,
    },
    crate::{
        app::node::UIEvent,
        prelude::{flow::CartesianFlowDirection, *},
        winitwgpu::{
            self,
            backend::{Node, Resources},
            pipeline::{graphics::GraphicsPipelineBuffers, RendererBuffers, Renderers},
            render_target::RenderTarget,
        },
    },
    pin_project::pin_project,
    winit::event::WindowEvent,
};

#[allow(non_snake_case)]
pub fn Image<A>(rect: Rect<f32, f32>, mut item: A) -> ImageNodeDescriptor<A::UIItem>
where
    A: LayoutItem + 'static,
{
    ImageNodeDescriptor {
        rect,
        content: item.lay(ParentHints {
            rect,
            current_flow_direction: CartesianFlowDirection::LeftToRight,
            current_cross_flow_direction: CartesianFlowDirection::TopToBottom,
            current_writing_flow_direction: CartesianFlowDirection::LeftToRight,
            current_writing_cross_flow_direction: CartesianFlowDirection::TopToBottom,
        }),
    }
}

pub struct ImageNodeDescriptor<A>
where
    A: 'static,
{
    rect: Rect<f32, f32>,
    content: A,
}

impl<T> Node for ImageNodeDescriptor<T>
where
    T: Send + Render + RenderGraphicDescriptor + 'static,
{
    type Reified = ImageNode;

    fn reify(
        self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        gpu_resources: &winitwgpu::backend::Resources,
        renderers: &mut Renderers,
    ) -> Self::Reified {
        ImageNode {
            rect: self.rect,
            content: Box::new(self.content),
            render_target: ImageRenderTarget::new(gpu_resources, self.rect.extent()),
            render_buffers: RendererBuffers {
                graphics_render_buffers: GraphicsPipelineBuffers::new(gpu_resources, T::QUAD_COUNT),
                text_render_buffers: TextPipelineBuffers::new(
                    gpu_resources,
                    &mut renderers.text_renderer,
                ),
            },
        }
    }
}

#[pin_project(project = ImageNodeProj)]
pub struct ImageNode {
    #[pin]
    content: Box<dyn Render>,
    rect: Rect<f32, f32>,
    render_buffers: RendererBuffers,
    render_target: ImageRenderTarget,
}

impl ReifiedNode for ImageNode {
    fn setup(&mut self, _gpu_resources: &Resources) {
        /* Do nothing */
    }

    fn handle_window_event(
        &mut self,
        gpu_resources: &mut Resources,
        pipelines: &mut Renderers,
        _window_id: winit::window::WindowId,
        _event: WindowEvent,
    ) {
        self.render(gpu_resources, pipelines);
        self.content.handle_ui_event(UIEvent::default());
    }

    fn poll_processors(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>> {
        let ImageNodeProj { mut content, .. } = self.project();

        let content: &mut _ = &mut **content;
        let content = unsafe { std::pin::Pin::new_unchecked(content) };

        let poll = content.poll_processors(cx);

        if let std::task::Poll::Ready(Some(())) = &poll {
            // Request Redraw!
        }

        poll
    }
}

impl ImageNode {
    fn render(&mut self, gpu_resources: &mut Resources, pipelines: &mut Renderers) {
        let size_bytes = 4 * 8 * self.rect.w as u64 * self.rect.h as u64;
        let size = self.render_target.image.texture.size();

        self.render_target.draw(
            self.content.as_mut(),
            gpu_resources,
            pipelines,
            &mut self.render_buffers,
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
}
