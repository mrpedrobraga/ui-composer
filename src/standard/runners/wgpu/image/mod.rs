use crate::standard::runners::wgpu::backend::WgpuResources;
use crate::standard::runners::wgpu::pipeline::graphics::RenderGraphic;
use crate::standard::runners::wgpu::pipeline::text::TextPipelineResources;
use crate::standard::runners::wgpu::pipeline::{
    RendererBuffers, UIContext, WgpuRenderers, graphics::GraphicsPipelineBuffers,
};
use crate::standard::runners::wgpu::render_target::{Render, RenderBuildingBlock, RenderTarget};
use crate::standard::runners::winitwgpu::runner::{Element, RuntimeElement};
use crate::geometry::layout::hints::ParentHints;
use crate::state::process::Pollable;
use image::{ImageBuffer, Rgba};
use std::pin::Pin;
use std::task::{Context, Poll};
use vek::Rect;
use wgpu::wgt::PollType;
use wgpu::{
    Origin3d, TexelCopyBufferInfo, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect,
};
use {
    super::texture::ImageRenderTarget,
    pin_project::pin_project,
    winit::event::WindowEvent,
};
use crate::app::composition::algebra::Bubble;
use crate::app::input::Event;
use crate::geometry::layout::flow::CartesianFlow;
use crate::geometry::layout::LayoutItem;

#[allow(non_snake_case)]
pub fn Image<A>(rect: Rect<f32, f32>, mut item: A) -> ImageNode<A::Content>
where
    A: LayoutItem + 'static,
{
    ImageNode {
        rect,
        content: item.lay(ParentHints {
            rect,
            current_flow_direction: CartesianFlow::LeftToRight,
            current_cross_flow_direction: CartesianFlow::TopToBottom,
            current_writing_flow_direction: CartesianFlow::LeftToRight,
            current_writing_cross_flow_direction: CartesianFlow::TopToBottom,
        }),
    }
}

pub struct ImageNode<Item> {
    rect: Rect<f32, f32>,
    content: Item,
}

impl<Item> Element for ImageNode<Item>
where
    Item: Render + Send + 'static,
{
    type Output = ImageNodeRe<Item::Output>;

    fn reify(
        self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        gpu_resources: &WgpuResources,
        renderers: WgpuRenderers,
    ) -> Self::Output {
        // TODO: Maybe store this?
        let mut reify_resources = UIContext { renderers };
        let content = self.content.reify(&mut reify_resources);

        ImageNodeRe {
            rect: self.rect,
            content,
            render_target: ImageRenderTarget::new(gpu_resources, self.rect.extent()),
            render_buffers: RendererBuffers {
                graphics_render_buffers: GraphicsPipelineBuffers::new(
                    gpu_resources,
                    Item::Output::QUAD_COUNT,
                ),
                _text_render_buffers: TextPipelineResources::new(
                    gpu_resources,
                    &mut reify_resources.renderers.text_renderer,
                ),
            },
        }
    }
}

#[pin_project(project = ImageNodeProj)]
pub struct ImageNodeRe<A: RenderBuildingBlock> {
    #[pin]
    content: A,
    rect: Rect<f32, f32>,
    render_buffers: RendererBuffers,
    render_target: ImageRenderTarget,
}

/* Use a different implementation of "Node" for Image Node that's detached from winit!  */
impl<A> Bubble<Event, bool> for ImageNodeRe<A>
where
    A: RenderBuildingBlock,
{
    fn bubble(&mut self, event: &mut Event) -> bool {
        self.content.bubble(event)
    }
}

impl<A> RuntimeElement for ImageNodeRe<A>
where
    A: RenderBuildingBlock,
{
    fn setup(&mut self, _gpu_resources: &WgpuResources) {
        /* Do nothing */
        println!("Image node was asked to be set up!");
    }

    fn handle_window_event(
        &mut self,
        _gpu_resources: &mut WgpuResources,
        _window_id: winit::window::WindowId,
        _event: WindowEvent,
    ) {
        // No event handling, it's an image!!!
    }
}

impl<A, Res> Pollable<Res> for ImageNodeRe<A>
where
    A: RenderBuildingBlock,
{
    fn poll(self: Pin<&mut Self>, _cx: &mut Context, _resources: &mut Res) -> Poll<Option<()>> {
        let ImageNodeProj { .. } = self.project();

        // let resources = ...;
        // let poll = content.poll(cx, resources);
        //
        // if let std::task::Poll::Ready(Some(())) = &poll {
        //     // Request Redraw!
        //     println!("Requesting that the image redraws!")
        // }
        //
        // poll
        Poll::Ready(Some(()))
    }
}

impl<A> ImageNodeRe<A>
where
    A: RenderBuildingBlock,
{
    #[allow(unused)]
    fn render(&mut self, gpu_resources: &mut WgpuResources, pipelines: &mut WgpuRenderers) {
        println!("Image Render Requested");

        let size_bytes = 4 * 8 * self.rect.w as u64 * self.rect.h as u64;
        let size = self.render_target.image.texture.size();

        self.render_target.draw(
            &mut self.content,
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
            TexelCopyTextureInfo {
                texture: &self.render_target.image.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            TexelCopyBufferInfo {
                buffer: buffer.as_ref(),
                layout: TexelCopyBufferLayout {
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
            gpu_resources
                .device
                .poll(PollType::Wait)
                .expect("Couldn't... wait?");
            rx.recv().unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            let v: Vec<_> = data
                .chunks_exact(4)
                .flat_map(|chunk| [chunk[2], chunk[1], chunk[0], chunk[3]])
                .collect();
            let img_buffer =
                ImageBuffer::<Rgba<u8>, _>::from_raw(size.width, size.height, v).unwrap();
            img_buffer.save("image.png").unwrap();
        }
        buffer.unmap();
    }
}
