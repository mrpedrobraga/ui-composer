use super::backend::GPUResources;
use crate::gpu::pipeline::orchestra_renderer::GraphicsPipelineBuffers;
use crate::gpu::pipeline::{RendererBuffers, Renderers};
use crate::ui::node::{ItemDescriptor, UIItem};
use futures_signals::signal_vec::MutableVec;
use vek::Rect;
use wgpu::{RenderPass, Texture};

pub struct VecItem<A: UIItem> {
    rect: Rect<f32, f32>,
    items: MutableVec<A>,
    render_buffers: Option<RendererBuffers>,
}

impl<A: UIItem + ItemDescriptor> VecItem<A> {
    pub fn new(rect: Rect<f32, f32>, items: MutableVec<A>) -> Self {
        Self {
            rect,
            items,
            render_buffers: None,
        }
    }

    pub fn initialize(&mut self, gpu_resources: &GPUResources) {
        if let None = self.render_buffers {
            self.render_buffers = Some(RendererBuffers {
                graphics_render_buffers: GraphicsPipelineBuffers::new(
                    gpu_resources,
                    self.items.lock_ref().len() * A::QUAD_COUNT,
                ),
            });
        }
    }
}

impl<A: ItemDescriptor + Sync> ItemDescriptor for VecItem<A> {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        Some(self.rect)
    }
}

impl<A: UIItem + ItemDescriptor + Sync> UIItem for VecItem<A> {
    fn handle_ui_event(&mut self, event: crate::ui::node::UIEvent) -> bool {
        // Handle UI events for each item!
        false
    }

    fn write_quads(&self, quad_buffer: &mut [crate::prelude::Graphic]) {
        // TODO: Write no quads.
    }

    fn prepare<'pass>(
        &'pass mut self,
        gpu_resources: &'pass GPUResources,
        pipelines: &'pass Renderers,
        mut render_pass: &mut RenderPass<'pass>,
        texture: &Texture,
    ) {
        self.initialize(gpu_resources);

        if let Some(render_buffers) = &mut self.render_buffers {
            let mut graphics_render_buffers = &mut render_buffers.graphics_render_buffers;
            let item_count = self.items.lock_ref().len();
            let quad_count = item_count * A::QUAD_COUNT;

            for idx in 0..item_count {
                let ui_tree = &self.items.lock_mut()[idx];
                ui_tree.write_quads(
                    &mut graphics_render_buffers.instance_buffer_cpu()
                        [(idx * A::QUAD_COUNT)..((idx + 1) * A::QUAD_COUNT)],
                );
            }
            graphics_render_buffers.write_to_gpu(gpu_resources);
            gpu_resources.queue.submit([]);

            render_pass.set_pipeline(&pipelines.graphics_renderer.pipeline);
            render_pass.set_bind_group(0, &pipelines.graphics_renderer.uniform_bind_group, &[]);
            render_pass
                .set_vertex_buffer(0, pipelines.graphics_renderer.mesh_vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                pipelines.graphics_renderer.mesh_index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            render_pass.set_vertex_buffer(1, graphics_render_buffers.instance_buffer());
            render_pass.set_vertex_buffer(1, graphics_render_buffers.instance_buffer());

            render_pass.draw_indexed(
                0..pipelines.graphics_renderer.mesh_index_count as u32,
                0,
                0..quad_count as u32,
            );
        }
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }

    fn poll_processors(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>> {
        // TODO: Poll the processors for my items!
        std::task::Poll::Ready(Some(()))
    }
}
