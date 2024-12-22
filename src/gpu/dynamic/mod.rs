use futures_signals::signal_vec::MutableVec;
use vek::{Rect, Rgb};
use wgpu::{RenderPass, Texture, TextureView};

use crate::{
    prelude::{Graphic, LayoutItem, RectExt as _},
    ui::node::{ItemDescriptor, UIItem},
};

use super::{backend::GPUResources, pipeline::GPURenderPipeline as _, world::UINodeRenderBuffers};

pub struct VecItem<A: UIItem> {
    rect: Rect<f32, f32>,
    items: MutableVec<A>,
    render_buffers: Option<UINodeRenderBuffers>,
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
            self.render_buffers = Some(UINodeRenderBuffers::new(
                gpu_resources,
                self.items.lock_ref().len() * A::QUAD_COUNT,
            ));
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

    fn nested_predraw<'pass>(
        &'pass mut self,
        gpu_resources: &'pass GPUResources,
        mut render_pass: &mut RenderPass<'pass>,
        texture: &Texture,
    ) {
        self.initialize(gpu_resources);

        if let Some(render_buffers) = &mut self.render_buffers {
            let item_count = self.items.lock_ref().len();
            let quad_count = item_count * A::QUAD_COUNT;

            for idx in 0..item_count {
                let ui_tree = &self.items.lock_mut()[idx];
                ui_tree.write_quads(
                    &mut render_buffers.instance_buffer_cpu()
                        [(idx * A::QUAD_COUNT)..((idx + 1) * A::QUAD_COUNT)],
                );
            }
            render_buffers.write_to_gpu(gpu_resources);
            gpu_resources.queue.submit([]);

            gpu_resources
                .main_pipeline
                .install_on_render_pass(&mut render_pass);
            render_pass.set_vertex_buffer(1, render_buffers.instance_buffer());

            render_pass.draw_indexed(
                0..gpu_resources.main_pipeline.mesh_index_count as u32,
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
