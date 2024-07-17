use crate::{
    alloc::{self, RenderModulePipeline, UIFragment},
    app::render_state::RenderTarget,
};
use bytemuck::{Pod, Zeroable};
use std::mem::size_of;
use wgpu::{util::DeviceExt, BufferAddress, BufferUsages};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Primitive {
    pub transform: [[f32; 4]; 4],
    pub color: [f32; 3],
}

impl UIFragment for Primitive {
    fn get_allocation_info(&self) -> alloc::AllocationInfo {
        alloc::AllocationInfo {
            buffer_size: size_of::<Self>() as u32,
            primitive_count: 1,
        }
    }

    fn push_allocation(&self, primitive_buffer: &mut Vec<u8>) {
        primitive_buffer.extend(bytemuck::cast_slice(&[*self]))
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

// Only generate a single render stack pipeline and share it across stacks.
pub fn get_main_render_stack_pipeline<'window>(
    window: std::sync::Arc<winit::window::Window>,
    surface: wgpu::Surface<'window>,
    adapter: &wgpu::Adapter,
    device: wgpu::Device,
) -> alloc::RenderModulePipeline<'window> {
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

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
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
        vertex_buffer: vertex_buffer,
        index_buffer: index_buffer,
        index_count: indices.len() as u32,
        render_texture: render_target,
        device,
    }
}
