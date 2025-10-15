use crate::app::building_blocks::Reifiable;
use crate::wgpu::backend::WgpuResources;
use crate::wgpu::render_target::{RenderWgpu, RenderTarget};
use wgpu::{CompareFunction, DepthBiasState, DepthStencilState, StencilState, TextureFormat};
use {
    super::{WgpuRenderer, RendererBuffers, WgpuRenderers},
    bytemuck::{Pod, Zeroable},
    graphic::Graphic,
    std::{mem::size_of, ops::Deref},
    vek::{Extent2, Mat4, Rect, Vec2, Vec3},
    wgpu::{
        BufferAddress, BufferUsages, ColorTargetState, RenderPass, Texture, util::DeviceExt as _,
    },
};

pub mod graphic;

#[doc(hidden)]
pub mod implementations;

/// Descriptor for some [RenderGraphic].
pub trait RenderGraphicDescriptor<Resources>:
    Reifiable<Resources, Reified: RenderGraphic>
{
    /// Gets the rectangle this primitive occupies, for rendering purposes.
    fn get_render_rect(&self) -> Option<Rect<f32, f32>>;
}

/// Trait for a [Reifiable] that can render graphics using this pipeline.
/// There's no trait bounds on this trait for convenience.
pub trait RenderGraphic {
    /// The number of quads this primitive pushes to render.
    const QUAD_COUNT: usize;

    /// Pushes quads to a quad buffer slice.
    fn write_quads(&self, quad_buffer: &mut [Graphic]);
}

/// The buffers that hold the soon-to-be-rendered UI.
pub struct GraphicsPipelineBuffers {
    instance_buffer_cpu: Vec<Graphic>,
    instance_buffer: wgpu::Buffer,
}

impl GraphicsPipelineBuffers {
    pub fn get_quad_count(&self) -> usize {
        self.instance_buffer_cpu.len()
    }

    /// Creates new buffers for the UI primitives to be drawn.
    pub fn new(gpu_resources: &WgpuResources, primitive_count: usize) -> Self {
        Self {
            instance_buffer_cpu: vec![Graphic::default(); primitive_count],
            instance_buffer: gpu_resources.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: size_of::<Graphic>() as u64 * primitive_count as u64,
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
        }
    }

    pub fn instance_buffer_cpu(&mut self) -> &mut [Graphic] {
        &mut self.instance_buffer_cpu[..]
    }

    pub fn instance_buffer(&mut self) -> wgpu::BufferSlice {
        self.instance_buffer.slice(..)
    }

    pub fn write_to_gpu(&mut self, gpu_resources: &WgpuResources) {
        gpu_resources.queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(self.instance_buffer_cpu.deref()),
        );
    }

    pub fn extend<I>(&mut self, gpu_resources: &WgpuResources, new_elements: I)
    where
        I: Iterator<Item = Graphic>,
    {
        let _ = new_elements;
        let _ = gpu_resources;
    }
}

pub struct OrchestraRenderer {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub mesh_vertex_buffer: wgpu::Buffer,
    pub mesh_index_buffer: wgpu::Buffer,
    pub mesh_index_count: usize,
    pub uniform_buffer: wgpu::Buffer,
    pub(crate) uniform_bind_group: wgpu::BindGroup,
}

impl WgpuRenderer for OrchestraRenderer {
    fn draw<R: RenderWgpu>(
        gpu_resources: &mut WgpuResources,
        renderers: &mut WgpuRenderers,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        ui_tree: &R,
        render_buffers: &mut RendererBuffers,
    ) {
        let _ = texture;
        let this = &mut renderers.graphics_renderer;
        let graphics_render_buffers = &mut render_buffers.graphics_render_buffers;

        let quad_count = graphics_render_buffers.get_quad_count();

        // TODO: Ponder on whether this is the best async way for flushing the uniforms.
        // Like, I think I might need _more_ than a single set of uniforms for peak
        // parallel rendering if I have multiple windows or multiple worlds.
        gpu_resources.queue.write_buffer(
            &this.uniform_buffer,
            0,
            bytemuck::cast_slice(&[Uniforms {
                world_to_wgpu_mat: container_size_to_wgpu_mat(render_target_size),
            }]),
        );

        ui_tree.write_quads(graphics_render_buffers.instance_buffer_cpu());
        //ui_tree.prepare(gpu_resources, pipelines, render_pass, &texture);

        if quad_count > 0 {
            graphics_render_buffers.write_to_gpu(gpu_resources);
            gpu_resources.queue.submit([]);

            render_pass.set_pipeline(&this.pipeline);
            render_pass.set_bind_group(0, &this.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, this.mesh_vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(this.mesh_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.set_vertex_buffer(1, graphics_render_buffers.instance_buffer());

            render_pass.draw_indexed(0..this.mesh_index_count as u32, 0, 0..quad_count as u32);
        }
    }
}

impl OrchestraRenderer {
    /// TODO: Make this a singleton.
    pub fn singleton<'a, Target>(
        adapter: &'a wgpu::Adapter,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        render_target_formats: &'a [Option<ColorTargetState>],
    ) -> Self
    where
        Target: RenderTarget,
    {
        let _ = queue;
        let _ = adapter;
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Render Stack Uniform Buffer"),
            size: size_of::<Uniforms>() as u64,
            mapped_at_creation: false,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Standard Uniform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Standard Uniform Bind Group"),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Standard Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });
        let shader = device.create_shader_module(wgpu::include_wgsl!(
            "./orchestra_render_pipeline_shader.wgsl"
        ));
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::buffer_layout(), Graphic::buffer_layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: render_target_formats,
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None, // TODO: Perhaps have some cache?
        });

        let (vertices, indices) = quad_mesh();

        let mesh_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices[..]),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        let mesh_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices[..]),
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
        });

        OrchestraRenderer {
            pipeline,
            mesh_vertex_buffer,
            mesh_index_buffer,
            mesh_index_count: indices.len(),
            uniform_buffer,
            uniform_bind_group,
        }
    }
}

impl Graphic {
    fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Self>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // transformation matrices
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[[f32; 4]; 1]>() as BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[[f32; 4]; 2]>() as BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[[f32; 4]; 3]>() as BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // color
                wgpu::VertexAttribute {
                    offset: size_of::<[[f32; 4]; 4]>() as BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // corner radii
                wgpu::VertexAttribute {
                    offset: (size_of::<[[f32; 4]; 4]>() + size_of::<[f32; 3]>()) as BufferAddress,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Uniforms {
    pub world_to_wgpu_mat: Mat4<f32>,
}

impl Uniforms {
    pub fn resize(&mut self, container_size: Extent2<f32>) {
        self.world_to_wgpu_mat = container_size_to_wgpu_mat(container_size);
    }
}

#[inline(always)]
pub fn container_size_to_wgpu_mat(Extent2 { w, h }: Extent2<f32>) -> Mat4<f32> {
    Mat4::identity()
        .scaled_3d(Vec3::new(2.0 / w, -2.0 / h, 1.0))
        .translated_2d(Vec2::new(-1.0, 1.0))
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    const fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Self>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

/// Returns the mesh for the quad used in the standard pipeline.
/// ```txt
/// (0,0)          (1,0)
/// *-------------*
/// |             |
/// |             |
/// |             |
/// |             |
/// *-------------*
/// (0, 0)         (1, 1)
/// ```
const fn quad_mesh() -> ([Vertex; 4], [u32; 6]) {
    (
        [
            Vertex {
                position: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.0, 1.0, 0.0],
            },
        ],
        [0, 1, 2, 2, 3, 0],
    )
}
