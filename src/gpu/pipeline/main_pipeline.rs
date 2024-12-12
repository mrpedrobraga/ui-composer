use bytemuck::{Pod, Zeroable};
use std::mem::size_of;
use vek::{Extent2, Mat4, Vec2, Vec3};
use wgpu::{util::DeviceExt as _, BufferAddress, BufferUsages, ColorTargetState};

use crate::{
    gpu::{engine::GPUResources, render_target::GPURenderTarget, world::UINodeRenderBuffers},
    ui::{graphics::Quad, node::UINode},
};

use super::GPURenderPipeline;

pub struct MainRenderPipeline {
    pipeline: wgpu::RenderPipeline,
    pub mesh_vertex_buffer: wgpu::Buffer,
    pub mesh_index_buffer: wgpu::Buffer,
    pub mesh_index_count: usize,
    pub uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl GPURenderPipeline for MainRenderPipeline {
    fn install_on_render_pass<'pass>(&'pass self, render_pass: &mut wgpu::RenderPass<'pass>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.mesh_vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.mesh_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    }
}

// Only generate a single render stack pipeline and share it across stacks.
pub fn main_render_pipeline<'a, Target>(
    adapter: &'a wgpu::Adapter,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    render_target_formats: &'a [Option<ColorTargetState>],
) -> MainRenderPipeline
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
    let shader = device.create_shader_module(wgpu::include_wgsl!("./main_shader.wgsl"));
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Vertex::buffer_layout(), Quad::buffer_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            compilation_options: Default::default(),
            targets: render_target_formats,
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None, // yet
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
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

    MainRenderPipeline {
        pipeline,
        mesh_vertex_buffer,
        mesh_index_buffer,
        mesh_index_count: indices.len(),
        uniform_buffer,
        uniform_bind_group,
    }
}

pub fn main_render_pipeline_draw(
    gpu_resources: &GPUResources,
    container_size: Extent2<f32>,
    view: wgpu::TextureView,
    content: &dyn UINode,
    render_artifacts: &UINodeRenderBuffers,
) {
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
                    r: 0.95,
                    g: 0.95,
                    b: 0.95,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
    });

    let quad_count = content.get_quad_count();

    if quad_count > 0 {
        // TODO: Flush uniforms here!
        gpu_resources.queue.write_buffer(
            &gpu_resources.main_pipeline.uniform_buffer,
            0,
            bytemuck::cast_slice(&[Uniforms {
                world_to_wgpu_mat: container_size_to_wgpu_mat(container_size),
            }]),
        );

        // TODO: Update the quads using a more efficient method;
        let mut quads = vec![crate::prelude::Quad::default(); quad_count];
        content.push_quads(&mut quads[..]);
        gpu_resources.queue.write_buffer(
            &render_artifacts.instance_buffer,
            0,
            bytemuck::cast_slice(&quads),
        );
        gpu_resources.queue.submit([]);

        // TODO: Allow partial renders of the UI...
        gpu_resources
            .main_pipeline
            .install_on_render_pass(&mut render_pass);
        render_pass.set_vertex_buffer(1, render_artifacts.instance_buffer.slice(..));

        render_pass.draw_indexed(
            0..gpu_resources.main_pipeline.mesh_index_count as u32,
            0,
            0..quads.len() as u32,
        );
    }

    drop(render_pass);

    gpu_resources
        .queue
        .submit(std::iter::once(encoder.finish()));
}

impl Quad {
    fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
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
                wgpu::VertexAttribute {
                    offset: size_of::<[[f32; 4]; 4]>() as BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x3,
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
    color: [f32; 3],
}

impl Vertex {
    const fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

/// Returns the mesh for the quad used in the standard pipeline.
/// ```
/// 0             1
/// *-------------*
/// |             |
/// |             |
/// |             |
/// |             |
/// *-------------*
/// 1
/// ```
const fn quad_mesh() -> ([Vertex; 4], [u32; 6]) {
    (
        [
            Vertex {
                position: [0.0, 0.0, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [1.0, 0.0, 0.0],
                color: [1.0, 1.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.0, 1.0, 0.0],
                color: [0.5, 0.5, 0.5],
            },
        ],
        [0, 1, 2, 2, 3, 0],
    )
}
