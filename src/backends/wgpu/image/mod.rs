use crate::app::building_blocks::BuildingBlock;
use crate::backends::wgpu::backend::WgpuResources;
use crate::backends::wgpu::pipeline::graphics::RenderGraphic;
use crate::backends::wgpu::pipeline::text::TextPipelineResources;
use crate::backends::wgpu::pipeline::{
    RendererBuffers, UIContext, WgpuRenderers, graphics::GraphicsPipelineBuffers,
};
use crate::backends::wgpu::render_target::{Render, RenderBuildingBlock, RenderTarget};
use crate::backends::winitwgpu::backend::{Node, NodeRe};
use crate::layout::hints::ParentHints;
use crate::state::process::Pollable;
use image::{ImageBuffer, Rgba};
use std::pin::Pin;
use std::task::{Context, Poll};
use wgpu::wgt::PollType;
use wgpu::{
    Origin3d, TexelCopyBufferInfo, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect,
};
use {
    super::texture::ImageRenderTarget,
    crate::prelude::{flow::CartesianFlowDirection, *},
    pin_project::pin_project,
    winit::event::WindowEvent,
};

#[allow(non_snake_case)]
pub fn Image<A>(rect: Rect<f32, f32>, mut item: A) -> ImageNode<A::Content>
where
    A: LayoutItem + 'static,
{
    ImageNode {
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

pub struct ImageNode<Item> {
    rect: Rect<f32, f32>,
    content: Item,
}

impl<Item> Node for ImageNode<Item>
where
    Item: Render + Send + 'static,
{
    type Reified = ImageNodeRe<Item::Reified>;

    fn reify(
        self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        gpu_resources: &WgpuResources,
        renderers: WgpuRenderers,
    ) -> Self::Reified {
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
                    Item::Reified::QUAD_COUNT,
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
impl<Res, A> BuildingBlock<Res> for ImageNodeRe<A>
where
    A: RenderBuildingBlock,
{
    fn handle_event(&mut self, event: Event) -> bool {
        self.content.handle_event(event)
    }
}

impl<A> NodeRe for ImageNodeRe<A>
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
