//! Empty for now but this will house different kinds of Textures that can be rendered onto quads!

use super::engine::GPUResources;

/// Color data that can be used by quads or materials to create advanced graphics.
/// Each implementer of Texture can generate data by its own method.
pub trait Texture {}

/// A texture that draws data from an image file.
pub struct ImageTexture {
    texture: wgpu::Texture,
}
impl ImageTexture {
    fn new(gpu_resources: &GPUResources) {
        let texture_size = 256u32;

        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: texture_size,
                height: texture_size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
        };
        let texture = gpu_resources.device.create_texture(&texture_desc);
        let texture_view = texture.create_view(&Default::default());
    }
}

impl Texture for ImageTexture {}

/// A texture that a world can draw to.
pub struct WorldTexture {
    texture: wgpu::Texture,
}
impl Texture for WorldTexture {}
