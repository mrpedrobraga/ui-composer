use std::{
    mem::size_of,
    ops::DerefMut,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Instant,
};

use futures_signals::signal::{Mutable, Signal, SignalExt};
use pin_project::pin_project;
use vek::{Extent2, Rect, Rgb};
use wgpu::{
    core::device::queue, BufferUsages, RenderPass, Surface, SurfaceConfiguration, TextureFormat,
    TextureView,
};
use winit::{
    dpi::{LogicalPosition, PhysicalPosition, PhysicalSize},
    event::WindowEvent,
    event_loop::{self, ActiveEventLoop},
    platform::x11::WindowAttributesExtX11,
    window::{Window, WindowButtons, WindowId},
};

use super::{
    engine::{GPUResources, LiveNode, Node},
    pipeline::{
        main_pipeline::{container_size_to_wgpu_mat, Uniforms},
        GPURenderPipeline,
    },
    render_target::{self, GPURenderTarget},
    view::{View, ViewNode},
};
use crate::ui::{
    graphics::Quad,
    layout::{LayoutHints, LayoutItem},
    node::{LiveUINode, UINode},
    react::{React, UISignalExt},
};

/// A node that describes the existence of a new window in the UI tree.
pub struct WindowNode<T: UINode> {
    state: WindowNodeState,
    content: T,
}

impl<T: UINode> WindowNode<T> {
    /// Consumes this window node and returns a new one with the set title.
    pub fn with_title<Str: Into<String>>(self, title: Str) -> WindowNode<T> {
        let WindowNodeState { title: _, size } = self.state;

        WindowNode {
            state: WindowNodeState {
                title: Mutable::new(title.into()),
                size,
            },
            content: self.content,
        }
    }

    /// Consumes this window node and returns a new one with a reactive title.
    /// The window's title will change every time this signal changes.
    pub fn with_reactive_title<Sig>(self, title_signal: Mutable<String>) -> WindowNode<T> {
        let WindowNodeState { title, size } = self.state;

        WindowNode {
            state: WindowNodeState {
                title: title_signal,
                size,
            },
            content: self.content,
        }
    }
}

/// Describes a new window with its contents and its own state.
#[allow(non_snake_case)]
pub fn Window<T>(item: T) -> WindowNode<impl UINode>
where
    T: LayoutItem + 'static,
{
    let state = WindowNodeState {
        size: Mutable::new(item.get_natural_size()),
        title: Mutable::new(String::new()),
    };

    let window_size_signal = state.size.signal();
    let minimum_size = item.get_natural_size();
    let item = state
        .size
        .signal()
        .map(move |window_size| {
            item.bake(LayoutHints {
                rect: Rect::new(0.0, 0.0, window_size.w, window_size.h),
            })
        })
        .into_ui();

    WindowNode {
        state,
        content: item,
    }
}

impl<T> Node for WindowNode<T>
where
    T: UINode + 'static,
{
    type LiveType = LiveWindowNode;

    /// Transforms a WindowNode, which merely describes a window, into an active node in an engine tree.
    fn reify(self, event_loop: &ActiveEventLoop, gpu_resources: &GPUResources) -> Self::LiveType {
        let window_default_size = self.state.size.get();

        let mut window = event_loop
            .create_window(
                winit::window::WindowAttributes::default()
                    .with_title(self.state.title.get_cloned())
                    .with_name("UI Composer App", "UI Composer App"),
            )
            .expect("Couldn't reify window node!");

        window.set_min_inner_size(Some(PhysicalSize::new(
            window_default_size.w,
            window_default_size.h,
        )));

        let window = std::sync::Arc::new(window);

        let render_artifacts = UINodeRenderingArtifacts {
            instance_buffer_cpu: vec![Quad::default(); T::QUAD_COUNT],
            instance_buffer: gpu_resources.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Instance Buffer for a Window"),
                size: (size_of::<Quad>() as u64 * T::QUAD_COUNT as u64),
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
        };

        LiveWindowNode {
            content: Box::new(self.content),
            state: self.state,
            render_artifacts,
            render_target: WindowRenderTarget::new(
                &gpu_resources,
                window.clone(),
                window_default_size.as_(),
            ),
            window,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct WindowAttributes<TitleSignal: Signal<Item = String>> {
    pub title: TitleSignal,
}

pub struct WindowNodeState {
    pub title: Mutable<String>,
    pub size: Mutable<Extent2<f32>>,
}

fn new_window_state(window_size: Extent2<f32>) -> WindowNodeState {
    WindowNodeState {
        size: Mutable::new(window_size),
        title: Mutable::new(String::new()),
    }
}

/// A live window which contains a UI tree inside.
#[pin_project(project = LiveWindowNodeProj)]
pub struct LiveWindowNode {
    #[pin]
    content: Box<dyn LiveUINode>,
    state: WindowNodeState,
    render_artifacts: UINodeRenderingArtifacts,
    render_target: WindowRenderTarget,
    window: Arc<Window>,
}

/// TODO: Move out of here and find a better name.
pub struct UINodeRenderingArtifacts {
    instance_buffer_cpu: Vec<Quad>,
    instance_buffer: wgpu::Buffer,
}

impl<'window> LiveNode for LiveWindowNode {
    fn handle_window_event(
        &mut self,
        gpu_resources: &GPUResources,
        window_id: WindowId,
        event: crate::ui::node::UIEvent,
    ) {
        if window_id == self.window.id() {
            match event {
                WindowEvent::Resized(new_size) => {
                    let new_size = Extent2::new(new_size.width, new_size.height);
                    self.render_target.resize(&gpu_resources, new_size);
                    self.state.size.set(new_size.as_());
                }
                WindowEvent::CloseRequested => {
                    // TODO: Handle closing of windows.
                    println!(
                        "[{}:{}] Find a better way to handle window close requests; Likely use monads or WindowState for this!",
                        file!(),
                        line!()
                    );
                    std::process::exit(0);
                }
                WindowEvent::RedrawRequested => {
                    self.redraw(gpu_resources);
                }
                _ => (),
            }
        }

        self.content.handle_ui_event(event);
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        // TODO: Figure out what do to with the result of this poll (as it might introduce a need for redrawing!!!);

        let LiveWindowNodeProj {
            mut content,
            render_artifacts,
            render_target,
            window,
            state,
        } = self.project();

        let content: &mut _ = &mut **content;
        let content = unsafe { Pin::new_unchecked(content) };

        let poll = content.poll_processors(cx);

        match &poll {
            Poll::Ready(_) => window.request_redraw(),
            _ => (),
        }

        poll
    }
}

impl LiveWindowNode {
    fn redraw(&mut self, gpu_resources: &GPUResources) {
        self.render_target
            .draw(gpu_resources, self.content.as_ref(), &self.render_artifacts);
    }
}

/// A render target that will be rendered to a window.
pub struct WindowRenderTarget {
    pub size: Extent2<u32>,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
}

impl WindowRenderTarget {
    pub fn new(gpu_resources: &GPUResources, target: Arc<Window>, size: Extent2<u32>) -> Self {
        let surface = gpu_resources.instance.create_surface(target).unwrap();
        let surface_config = surface
            .get_default_config(&gpu_resources.adapter, size.w, size.h)
            .unwrap();

        Self {
            size,
            surface,
            surface_config,
        }
    }
}

impl GPURenderTarget for WindowRenderTarget {
    fn resize(&mut self, gpu_resources: &GPUResources, new_size: Extent2<u32>) {
        self.surface_config = self
            .surface
            .get_default_config(&gpu_resources.adapter, new_size.w, new_size.h)
            .unwrap();
        self.surface
            .configure(&gpu_resources.device, &self.surface_config);
        self.size = new_size;
    }

    fn draw(
        &mut self,
        gpu_resources: &GPUResources,
        content: &dyn LiveUINode,
        render_artifacts: &UINodeRenderingArtifacts,
    ) {
        let texture = self
            .surface
            .get_current_texture()
            .expect("Error retrieving the current texture.");
        let view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

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
                    world_to_wgpu_mat: container_size_to_wgpu_mat(self.size.as_()),
                }]),
            );

            // TODO: Flush primitives to GPU here!
            let mut quads = vec![Quad::default(); quad_count];

            content.push_quads(&mut quads[..]);

            let dummy_primitives = gpu_resources.queue.write_buffer(
                &render_artifacts.instance_buffer,
                0,
                bytemuck::cast_slice(&quads),
            );
            gpu_resources.queue.submit([]);

            // TODO: Allow partial renders of the UI...
            gpu_resources.main_pipeline.apply_onto(&mut render_pass);
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
        texture.present();
    }

    fn get_texture_format(&self) -> TextureFormat {
        self.surface_config.format
    }
}
