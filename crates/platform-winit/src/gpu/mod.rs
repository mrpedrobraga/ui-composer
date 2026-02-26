use ui_composer_math::prelude::Size2;

pub struct Gpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

/// A connection with the gpu.
///
/// [Learn more](https://webgpufundamentals.org/webgpu/lessons/webgpu-fundamentals.html)
impl Gpu {
    pub async fn new() -> Self {
        // First, we create an `Instance` which is the entry point
        // into talking with the GPU.
        let instance: wgpu::Instance = Default::default();

        // We ask for a handle to a specific physical device that's compatible with our preferences.
        // We use it to request the underlying device.
        let adapter: wgpu::Adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                // TODO: Allow hinting of a compatible surface. In WebGPU, this is strictly required!
                compatible_surface: None,
            })
            .await
            .expect("Failed to acquire adapter!");

        // `device` - Open connection to a graphics and/or compute device. We use it to acquire resources like textures.
        // `queue` - It's like a `Sender` that allows sending commands to the GPU to change its state.
        // We use the queue to draw pretty pictures :-)
        let (device, queue): (wgpu::Device, wgpu::Queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("UI Composer Winit Main Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::Performance,
                // TODO: Handle tracing, probably!
                trace: Default::default(),
                experimental_features: Default::default(),
            })
            .await
            .expect("Failed to create device and queue!");

        Gpu { device, queue }
    }
}

#[allow(async_fn_in_trait)]
pub trait RenderTarget {
    async fn resize(&mut self, gpu: &Gpu, new_size: Size2<u32>);
}
