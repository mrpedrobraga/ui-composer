use crate::runners::winit::gpu::{Gpu, RenderTarget};
use std::sync::Arc;
use vek::Extent2;
use wgpu::{Surface, Texture, TextureDimension, TextureFormat, TextureUsages};
use winit::window::{Window, WindowId};

pub struct WindowRenderTarget {
    pub size: Extent2<u32>,
    pub surface: Surface<'static>,
    pub depth_texture: wgpu::Texture,
}

impl WindowRenderTarget {
    /// Creates a new `RenderTarget` which renders to a window.
    pub fn new(gpu: &Gpu, window: Arc<Window>) -> Self {
        let window_id = window.id();
        let size = window.inner_size();
        let size = Extent2::new(size.width, size.height);
        let surface = wgpu::Instance::default()
            .create_surface(window)
            .expect("Failed to create surface for window!");
        let depth_texture = Self::new_depth_texture(gpu, &size);

        Self {
            size,
            surface,
            depth_texture,
        }
    }

    fn new_depth_texture(gpu: &Gpu, size: &Extent2<u32>) -> Texture {
        let depth_texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("UI Composer Winit Window Depth Texture.")),
            size: wgpu::Extent3d {
                width: size.w,
                height: size.h,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            // TODO: Maybe use ints for depth in 2D?
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        depth_texture
    }
}

impl RenderTarget for WindowRenderTarget {
    async fn resize(&mut self, gpu: &Gpu, new_size: Extent2<u32>) {
        let adapter = wgpu::Instance::default()
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&self.surface),
                ..Default::default()
            })
            .await
            .expect("Failed to request new adapter!");
        let surface_config = self
            .surface
            .get_default_config(&adapter, new_size.w, new_size.h)
            .expect("Failed to get new configuration for surface.");
        self.surface.configure(&gpu.device, &surface_config);
        self.depth_texture = Self::new_depth_texture(gpu, &new_size);
        self.size = new_size;
    }
}
