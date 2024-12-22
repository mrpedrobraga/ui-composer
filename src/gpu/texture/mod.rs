//! Empty for now but this will house different kinds of Textures that can be rendered onto quads!

use vek::Extent2;

use super::{
    backend::GPUResources,
    pipeline::{orchestra_render_pipeline::OrchestraRenderPipeline, GPURenderPipeline},
    render_target::GPURenderTarget,
};

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
        gpu_resources: &GPUResources,
        content: &mut dyn crate::ui::node::UIItem,
        render_artifacts: &mut super::world::UINodeRenderBuffers,
    ) {
        let texture = &self.image.texture;
        let size = self.image.texture.size();

        OrchestraRenderPipeline::draw(
            gpu_resources,
            Extent2::new(size.width as f32, size.height as f32),
            &texture,
            content,
            render_artifacts,
        );

        // TODO: Here we would "present" the texture.
        // In this case the idea is to notify whoever is
        // holding this image texture that its contents changed so that it can
        // redraw itself;
    }

    fn get_texture_format(&self) -> wgpu::TextureFormat {
        self.image.texture.format()
    }
}
