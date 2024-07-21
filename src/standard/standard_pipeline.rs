use std::mem::size_of;

use crate::app::engine::RenderTarget;
use bytemuck::{Pod, Zeroable};
use wgpu::{util::DeviceExt as _, BufferAddress, BufferUsages};

use super::{primitive::Primitive, render::tuple_render_module::RenderModulePipeline};

// Only generate a single render stack pipeline and share it across stacks.
pub fn get_main_render_stack_pipeline<'a, 'window>(
    window: std::sync::Arc<winit::window::Window>,
    surface: wgpu::Surface<'window>,
    adapter: &'a wgpu::Adapter,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
) -> RenderModulePipeline<'window> {
    let surface_config = surface
        .get_default_config(
            &adapter,
            window.inner_size().width,
            window.inner_size().height,
        )
        .unwrap();
    surface.configure(&device, &surface_config);

    let render_target = RenderTarget {
        size: window.inner_size(),
        surface,
        surface_config,
    };

    let swapchain_capabilities = render_target.surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let uniforms = StandardUniform {
        world_to_wgpu_mat: world_to_wgpu_mat(window.inner_size()),
    };

    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Render Stack Uniform Buffer"),
        contents: bytemuck::cast_slice(&[uniforms]),
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
    let shader = device.create_shader_module(wgpu::include_wgsl!("./standard-shader.wgsl"));
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Vertex::buffer_layout(), Primitive::buffer_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            compilation_options: Default::default(),
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None, // yet
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    let (vertices, indices) = get_quad_mesh();

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&vertices[..]),
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&indices[..]),
        usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
    });

    RenderModulePipeline {
        id: 0,
        pipeline: render_pipeline,
        vertex_buffer,
        index_buffer,
        index_count: indices.len() as u32,
        render_texture: render_target,
        uniforms,
        uniform_buffer,
        uniform_bind_group,
    }
}

impl Primitive {
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
pub struct StandardUniform {
    world_to_wgpu_mat: [[f32; 4]; 4],
}

impl StandardUniform {
    pub fn resize(&mut self, container_size: winit::dpi::PhysicalSize<u32>) {
        self.world_to_wgpu_mat = world_to_wgpu_mat(container_size);
    }
}

fn world_to_wgpu_mat(window_size: winit::dpi::PhysicalSize<u32>) -> [[f32; 4]; 4] {
    let ww = window_size.width as f32;
    let wh = window_size.height as f32;

    return [
        [2.0 / ww, 0.0, 0.0, 0.0],
        [0.0, -2.0 / wh, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-1.0, 1.0, 0.0, 1.0],
    ];
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
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
fn get_quad_mesh() -> (Vec<Vertex>, Vec<u32>) {
    (
        vec![
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
        vec![0, 1, 2, 2, 3, 0],
    )
}
