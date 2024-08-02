use std::sync::Arc;

use vek::Extent2;
use wgpu::{Surface, SurfaceConfiguration, SurfaceTarget};
use winit::dpi::PhysicalSize;

/// Describes a RenderTarget that a render module can render to.
/// TODO: Move this out of here.
pub struct GPURenderTarget<'window> {
    pub size: Extent2<u32>,
    pub surface: Surface<'window>,
    pub surface_config: SurfaceConfiguration,
}

impl<'window> GPURenderTarget<'window> {
    pub fn new(
        instance: &wgpu::Instance,
        adapter: &wgpu::Adapter,
        target: impl Into<SurfaceTarget<'window>>,
        size: Extent2<u32>,
    ) -> Self {
        let surface = instance.create_surface(target).unwrap();
        let surface_config = surface
            .get_default_config(&adapter, size.w, size.h)
            .unwrap();

        Self {
            size,
            surface,
            surface_config,
        }
    }

    pub fn resize(
        &mut self,
        device: &wgpu::Device,
        adapter: &wgpu::Adapter,
        new_size: Extent2<u32>,
    ) {
        self.surface_config = self
            .surface
            .get_default_config(&adapter, new_size.w, new_size.h)
            .unwrap();
        self.surface.configure(&device, &self.surface_config);
    }
}
