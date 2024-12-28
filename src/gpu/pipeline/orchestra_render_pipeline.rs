use bytemuck::{Pod, Zeroable};
use std::mem::size_of;
use vek::{Extent2, Mat4, Vec2, Vec3};
use wgpu::{
    util::DeviceExt as _, BufferAddress, BufferUsages, ColorTargetState, RenderPass, Texture,
};

use super::GPURenderPipeline;
use crate::gpu::backend::{GPUResources, Pipelines};
use crate::gpu::world::UINodeRenderBuffers;
use crate::prelude::UIItem;
use crate::{gpu::render_target::GPURenderTarget, ui::graphics::Graphic};

pub struct OrchestraRenderPipeline {
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub mesh_vertex_buffer: wgpu::Buffer,
    pub mesh_index_buffer: wgpu::Buffer,
    pub mesh_index_count: usize,
    pub uniform_buffer: wgpu::Buffer,
    pub(crate) uniform_bind_group: wgpu::BindGroup,
}

impl GPURenderPipeline for OrchestraRenderPipeline {
    fn draw(
        gpu_resources: &mut GPUResources,
        pipelines: &mut Pipelines,
        render_target_size: Extent2<f32>,
        texture: &Texture,
        render_pass: &mut RenderPass,
        ui_tree: &mut dyn UIItem,
        render_buffers: &mut UINodeRenderBuffers,
    ) {
        let this = &mut pipelines.graphics_pipeline;
        let quad_count = render_buffers.get_quad_count();

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

        ui_tree.write_quads(render_buffers.instance_buffer_cpu());
        //ui_tree.prepare(gpu_resources, pipelines, render_pass, &texture);

        if quad_count > 0 {
            render_buffers.write_to_gpu(gpu_resources);
            gpu_resources.queue.submit([]);

            render_pass.set_pipeline(&this.pipeline);
            render_pass.set_bind_group(0, &this.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, this.mesh_vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(this.mesh_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.set_vertex_buffer(1, render_buffers.instance_buffer());

            render_pass.draw_indexed(0..this.mesh_index_count as u32, 0, 0..quad_count as u32);
        }
    }
}

impl OrchestraRenderPipeline {
    /// TODO: Make this a singleton.
    pub fn singleton<'a, Target>(
        adapter: &'a wgpu::Adapter,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        render_target_formats: &'a [Option<ColorTargetState>],
    ) -> OrchestraRenderPipeline
    where
        Target: GPURenderTarget,
    {
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Render Stack Uniform Buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
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
            depth_stencil: None, // yet
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

        OrchestraRenderPipeline {
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
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
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
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
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
