//! Empty for now but this will house different kinds of Textures that can be rendered onto quads!

use super::{backend::GPUResources, render_target::GPURenderTarget};
use crate::gpu::pipeline::graphics::OrchestraRenderer;
use crate::gpu::pipeline::text::GlyphonTextRenderer;
use crate::gpu::pipeline::{GPURenderer, RendererBuffers, Renderers};
use vek::Extent2;

/// Color data that can be used by quads or materials to create advanced graphics.
/// Each implementer of Texture can generate data by its own method.
pub trait Texture {}

/// A texture that draws data from an image file.
pub struct ImageTexture {
    pub texture: wgpu::Texture,
}
impl ImageTexture {
    fn new(gpu_resources: &GPUResources, size: Extent2<f32>) -> Self {
        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: size.w as u32,
                height: size.h as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[wgpu::TextureFormat::Bgra8UnormSrgb],
        };
        let texture = gpu_resources.device.create_texture(&texture_desc);

        Self { texture }
    }

    fn resize(&mut self, gpu_resources: &GPUResources, new_size: vek::Extent2<u32>) {}
}

impl Texture for ImageTexture {}

/// A texture that a world can draw to.
pub struct WorldTexture {
    texture: wgpu::Texture,
}
impl Texture for WorldTexture {}

pub struct ImageRenderTarget {
    pub image: ImageTexture,
}

impl ImageRenderTarget {
    pub fn new(gpu_resources: &GPUResources, size: Extent2<f32>) -> Self {
        Self {
            image: ImageTexture::new(gpu_resources, size),
        }
    }
}

impl GPURenderTarget for ImageRenderTarget {
    fn resize(&mut self, gpu_resources: &GPUResources, new_size: vek::Extent2<u32>) {
        self.image.resize(gpu_resources, new_size)
    }

    fn draw(
        &mut self,
        content: &mut dyn crate::ui::node::UIItem,
        gpu_resources: &mut GPUResources,
        pipelines: &mut Renderers,
        render_artifacts: &mut RendererBuffers,
    ) {
        let texture = &self.image.texture;
        let size = self.image.texture.size();
        let size = Extent2::new(size.width as f32, size.height as f32);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            gpu_resources
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command Encoder"),
                });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.015,
                        g: 0.015,
                        b: 0.015,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        OrchestraRenderer::draw(
            gpu_resources,
            pipelines,
            size,
            &texture,
            &mut render_pass,
            content,
            render_artifacts,
        );

        GlyphonTextRenderer::draw(
            gpu_resources,
            pipelines,
            size,
            &texture,
            &mut render_pass,
            content,
            render_artifacts,
        );

        gpu_resources
            .queue
            .submit(std::iter::once(encoder.finish()));
        // TODO: Here we would "present" the texture.
        // In this case the idea is to notify whoever is
        // holding this image texture that its contents changed so that it can
        // redraw itself;
    }

    fn get_texture_format(&self) -> wgpu::TextureFormat {
        self.image.texture.format()
    }
}
