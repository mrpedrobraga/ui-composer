use crate::app::primitives::PollProcessors;
use crate::wgpu::backend::Resources;
use crate::wgpu::pipeline::{graphics::GraphicsPipelineBuffers, RendererBuffers, Renderers};
use crate::wgpu::pipeline::{
    graphics::{graphic::Graphic, RenderGraphic, RenderGraphicDescriptor},
    text::TextPipelineBuffers,
};
use {
    crate::{app::primitives::Primitive, prelude::Event},
    futures_signals::signal_vec::MutableVec,
    vek::Rect,
    wgpu::{RenderPass, Texture},
};

pub struct VecItem<A: Primitive> {
    rect: Rect<f32, f32>,
    items: MutableVec<A>,
    render_buffers: Option<RendererBuffers>,
}

impl<A: Primitive + RenderGraphicDescriptor> VecItem<A> {
    pub fn new(rect: Rect<f32, f32>, items: MutableVec<A>) -> Self {
        Self {
            rect,
            items,
            render_buffers: None,
        }
    }

    pub fn initialize(&mut self, gpu_resources: &Resources, renderers: &mut Renderers) {
        if self.render_buffers.is_none() {
            self.render_buffers = Some(RendererBuffers {
                graphics_render_buffers: GraphicsPipelineBuffers::new(
                    gpu_resources,
                    self.items.lock_ref().len() * A::QUAD_COUNT,
                ),
                text_render_buffers: TextPipelineBuffers::new(
                    gpu_resources,
                    &mut renderers.text_renderer,
                ),
            });
        }
    }
}

impl<A: RenderGraphicDescriptor + Primitive + Sync> RenderGraphicDescriptor for VecItem<A> {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        Some(self.rect)
    }
}
impl<A: RenderGraphicDescriptor + Primitive + Sync> RenderGraphic for VecItem<A> {
    fn write_quads(&self, _quad_buffer: &mut [Graphic]) {
        // TODO: Write no quads.
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}
impl<A: RenderGraphicDescriptor + Primitive + Sync> Primitive for VecItem<A> {
    fn handle_event(&mut self, _event: Event) -> bool {
        // Handle UI events for each item!
        false
    }
}

impl<A: RenderGraphicDescriptor + Primitive + Sync> PollProcessors for VecItem<A> {
    fn poll_processors(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>> {
        // TODO: Poll the processors for my items!
        std::task::Poll::Ready(Some(()))
    }
}

#[allow(unused)]
fn prepare<'pass, A: Primitive + RenderGraphicDescriptor + Sync>(
    me: &mut VecItem<A>,
    gpu_resources: &'pass Resources,
    pipelines: &'pass Renderers,
    mut render_pass: &mut RenderPass<'pass>,
    texture: &Texture,
) {
    if let Some(render_buffers) = &mut me.render_buffers {
        let mut graphics_render_buffers = &mut render_buffers.graphics_render_buffers;
        let item_count = me.items.lock_ref().len();
        let quad_count = item_count * A::QUAD_COUNT;

        for idx in 0..item_count {
            let ui_tree = &me.items.lock_mut()[idx];
            ui_tree.write_quads(
                &mut graphics_render_buffers.instance_buffer_cpu()
                    [(idx * A::QUAD_COUNT)..((idx + 1) * A::QUAD_COUNT)],
            );
        }
        graphics_render_buffers.write_to_gpu(gpu_resources);
        gpu_resources.queue.submit([]);

        render_pass.set_pipeline(&pipelines.graphics_renderer.pipeline);
        render_pass.set_bind_group(0, &pipelines.graphics_renderer.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, pipelines.graphics_renderer.mesh_vertex_buffer.slice(..));
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
