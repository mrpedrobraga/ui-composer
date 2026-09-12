use ui_composer_math::glamour::{Matrix4, Size2};
use wgpu::util::DeviceExt;

use crate::{gpu::Gpu, window::effect_handling::QuadInstance};

#[allow(async_fn_in_trait)]
pub trait RenderTarget {
    async fn resize(&mut self, gpu: &Gpu, new_size: Size2<u32>);

    /// Returns a texture set useful for rendering!
    fn texture_set(&self) -> TextureSet;
}

pub struct TextureSet {
    pub surface_texture: Option<wgpu::SurfaceTexture>,
    pub albedo: wgpu::Texture,
    pub depth: wgpu::Texture,
}

pub struct RenderResources {
    pub gpu: Gpu,

    pub uniforms: RenderPipelineUniforms,
    pub uniforms_gpu: wgpu::Buffer,

    pub quads: Vec<QuadInstance>,
    pub quads_buffer: wgpu::Buffer,

    pub bind_group: wgpu::BindGroup,
}

impl RenderResources {
    pub fn new(gpu: Gpu, pipeline: &RenderPipeline) -> Self {
        let uniforms = RenderPipelineUniforms::new();
        let uniforms_gpu =
            gpu.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Quads Pipeline Uniform Buffer"),
                    contents: bytemuck::cast_slice(&[uniforms]),
                    usage: wgpu::BufferUsages::UNIFORM
                        | wgpu::BufferUsages::COPY_DST,
                });
        let quads = vec![QuadInstance::default(); 2];
        let quads_buffer =
            gpu.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Quads Pipeline Quad Data Buffer"),
                    contents: bytemuck::cast_slice(quads.as_slice()),
                    usage: wgpu::BufferUsages::STORAGE
                        | wgpu::BufferUsages::COPY_DST,
                });
        let bind_group =
            gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Quads Pipeline Bind Group"),
                layout: &pipeline.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniforms_gpu.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: quads_buffer.as_entire_binding(),
                    },
                ],
            });

        RenderResources {
            gpu,
            uniforms,
            uniforms_gpu,
            quads,
            quads_buffer,
            bind_group,
        }
    }

    /// Synchronizes the data between the CPU and the GPU.
    pub fn sync(&self, gpu: &Gpu) {
        // TODO: Only send what changed?
        gpu.queue.write_buffer(
            &self.uniforms_gpu,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
        gpu.queue.write_buffer(
            &self.quads_buffer,
            0,
            bytemuck::cast_slice(self.quads.as_slice()),
        );
        gpu.queue.submit(std::iter::empty());
    }
}

pub struct RenderPipeline {
    pub(crate) wgpu_pipeline: wgpu::RenderPipeline,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RenderPipelineUniforms {
    pub view_matrix: Matrix4<f32>,
}

impl RenderPipelineUniforms {
    pub fn new() -> Self {
        Self {
            view_matrix: ui_composer_math::glamour::Matrix4::orthographic_rh(
                0.0,
                640_f32,
                360_f32,
                0.0,
                -1.0,
                1.0,
            ),
        }
    }
}

impl RenderPipeline {
    pub fn new(gpu: &Gpu, target_format: wgpu::TextureFormat) -> Self {
        let shader = gpu
            .device
            .create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let bind_group_layout = gpu.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Quads Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage {
                                read_only: true,
                            },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            },
        );

        let pipeline_layout = gpu.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Quads Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                immediate_size: 0,
            },
        );

        let wgpu_pipeline = gpu.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Quads Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options:
                        wgpu::PipelineCompilationOptions::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options:
                        wgpu::PipelineCompilationOptions::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: target_format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true, //Some(true),
                    depth_compare: wgpu::CompareFunction::LessEqual, //Some(wgpu::CompareFunction::Less),
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview_mask: None, // TODO: Use for VR maybe?
                cache: None,
            },
        );

        RenderPipeline {
            wgpu_pipeline,
            bind_group_layout,
        }
    }
}

pub fn render<R>(
    target: &R,
    pipeline: &RenderPipeline,
    resources: &RenderResources,
) where
    R: RenderTarget,
{
    let TextureSet {
        surface_texture,
        albedo,
        depth,
    } = target.texture_set();

    let mut command_encoder = resources.gpu.device.create_command_encoder(
        &wgpu::CommandEncoderDescriptor {
            label: Some("Window Command Encoder"),
        },
    );

    let albedo_view = albedo.create_view(&Default::default());
    let depth_view = depth.create_view(&Default::default());

    let mut pass = render_pass(&mut command_encoder, albedo_view, depth_view);

    /* Draw all the wonderful, wonderful quads */
    pass.set_pipeline(&pipeline.wgpu_pipeline);
    pass.set_bind_group(0, Some(&resources.bind_group), &[]);
    pass.draw(0..6, 0..(resources.quads.len() as u32));

    drop(pass);

    resources
        .gpu
        .queue
        .submit(std::iter::once(command_encoder.finish()));

    // Present the texture!
    if let Some(surface_texture) = surface_texture {
        surface_texture.present();
        println!("Presenting!")
    }
}

fn render_pass<'render>(
    command_encoder: &'render mut wgpu::CommandEncoder,
    albedo: wgpu::TextureView,
    depth: wgpu::TextureView,
) -> wgpu::RenderPass<'render> {
    // TODO: Make this configurable per window or per app or both even.
    let clear_color = wgpu::Color {
        r: 0.9,
        g: 0.9,
        b: 0.9,
        a: 1.0,
    };

    let color_attachment = wgpu::RenderPassColorAttachment {
        view: &albedo,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(clear_color),
            store: wgpu::StoreOp::Store,
        },
        depth_slice: None,
        resolve_target: None,
    };

    let depth_stencil_attachment = wgpu::RenderPassDepthStencilAttachment {
        view: &depth,
        depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.0),
            store: wgpu::StoreOp::Store,
        }),
        stencil_ops: None,
    };

    command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Main Render Pass"),
        color_attachments: &[Some(color_attachment)],
        depth_stencil_attachment: Some(depth_stencil_attachment),
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None, // TODO: Use this for VR ?
    })
}
