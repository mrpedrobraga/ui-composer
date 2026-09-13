use crate::gpu::Gpu;
use crate::render::RenderTarget;
use crate::render::TextureSet;
use std::sync::Arc;
use ui_composer_math::prelude::Size2;

/// The render target a window will draw to in order to show its elements in a [window](winit::window::Window).
pub struct WindowRenderTarget {
    pub size: Size2<u32>,
    pub surface: wgpu::Surface<'static>,
    pub depth_texture: wgpu::Texture,
}

impl WindowRenderTarget {
    /// Creates a new `RenderTarget` which renders to a window.
    pub fn new(gpu: &Gpu, window: Arc<winit::window::Window>) -> Self {
        let size = window.inner_size();
        let size = Size2::new(size.width, size.height);
        let surface = gpu
            .instance
            .create_surface(window)
            .expect("Failed to create surface for window!");
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            color_space: wgpu::SurfaceColorSpace::Auto,
        };
        surface.configure(&gpu.device, &surface_config);
        let depth_texture = Self::new_depth_texture(gpu, &size);

        Self {
            size,
            surface,
            depth_texture,
        }
    }

    pub(crate) fn new_depth_texture(gpu: &Gpu, size: &Size2<u32>) -> wgpu::Texture {
        gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("UI Composer Winit Window Depth Texture."),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // TODO: Maybe use ints for depth in 2D?
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
    }
}

impl RenderTarget for WindowRenderTarget {
    fn resize(&mut self, gpu: &Gpu, new_size: Size2<u32>) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: new_size.width,
            height: new_size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            color_space: wgpu::SurfaceColorSpace::Auto,
        };
        self.surface.configure(&gpu.device, &surface_config);
        self.depth_texture = Self::new_depth_texture(gpu, &new_size);
        self.size = new_size;
    }

    fn texture_set(&self) -> TextureSet {
        let surface_texture = self.surface.get_current_texture();
        match surface_texture {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => {
                let albedo = surface_texture.texture.clone();

                TextureSet {
                    surface_texture: Some(surface_texture),
                    albedo,
                    depth: self.depth_texture.clone(),
                }
            }
            _ => {
                panic!("No surface available?");
            }
        }
    }
}
