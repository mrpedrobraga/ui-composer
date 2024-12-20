use std::{
    mem::size_of,
    ops::DerefMut,
    pin::Pin,
    process::ExitCode,
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
    backend::{GPUResources, Node, NodeDescriptor},
    pipeline::{
        orchestra_render_pipeline::{
            container_size_to_wgpu_mat, OrchestraRenderPipeline, Uniforms,
        },
        GPURenderPipeline,
    },
    render_target::{self, GPURenderTarget},
    view::{View, ViewNode},
    world::UINodeRenderBuffers,
};
use crate::ui::{
    graphics::Quad,
    layout::{LayoutHints, LayoutItem},
    node::{UINode, UINodeDescriptor},
    react::{React, UISignalExt},
};

/// A node that describes the existence of a new window in the UI tree.
pub struct WindowNodeDescriptor<T: UINodeDescriptor> {
    state: WindowNodeState,
    content: T,
}

impl<T: UINodeDescriptor> WindowNodeDescriptor<T> {
    /// Consumes this window node and returns a new one with the set title.
    pub fn with_title<Str: Into<String>>(self, title: Str) -> WindowNodeDescriptor<T> {
        let WindowNodeState { title: _, size } = self.state;

        WindowNodeDescriptor {
            state: WindowNodeState {
                title: Mutable::new(title.into()),
                size,
            },
            content: self.content,
        }
    }

    /// Consumes this window node and returns a new one with a reactive title.
    /// The window's title will change every time this signal changes.
    pub fn with_reactive_title<Sig>(
        self,
        title_signal: Mutable<String>,
    ) -> WindowNodeDescriptor<T> {
        let WindowNodeState { title, size } = self.state;

        WindowNodeDescriptor {
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
pub fn Window<T>(item: T) -> WindowNodeDescriptor<impl UINodeDescriptor>
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
        .collect_ui();

    WindowNodeDescriptor {
        state,
        content: item,
    }
}

impl<T> NodeDescriptor for WindowNodeDescriptor<T>
where
    T: UINodeDescriptor + 'static,
{
    type ReifiedType = WindowNode;

    /// Transforms a WindowNode, which merely describes a window, into an active node in an engine tree.
    fn reify(
        self,
        event_loop: &ActiveEventLoop,
        gpu_resources: &GPUResources,
    ) -> Self::ReifiedType {
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

        let render_buffers = UINodeRenderBuffers::new(gpu_resources, T::QUAD_COUNT);

        WindowNode {
            content: Box::new(self.content),
            state: self.state,
            content_buffers: render_buffers,
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
#[pin_project(project = WindowNodeProj)]
pub struct WindowNode {
    #[pin]
    state: WindowNodeState,
    window: Arc<Window>,
    content: Box<dyn UINode>,
    content_buffers: UINodeRenderBuffers,
    render_target: WindowRenderTarget,
}

impl<'window> Node for WindowNode {
    fn setup(&mut self, gpu_resources: &GPUResources) {}

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
                        "[{}:{}] Closing a window, at the moment, kills the process. This behaviour *will* change.",
                        file!(),
                        line!()
                    );
                    // Close request shouldn't be handled by the window, but by a "WindowManager" node of some sorts.
                    // The window is then closed by having all references to it dropped.
                    // Of cours the WindowManager needs to know if the window *can* be closed - if there's any process impeding it from closing,
                    // but that's a different story.
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

        let WindowNodeProj {
            mut content,
            content_buffers,
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

impl WindowNode {
    fn redraw(&mut self, gpu_resources: &GPUResources) {
        self.render_target.draw(
            gpu_resources,
            self.content.as_mut(),
            &mut self.content_buffers,
        );
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
        content: &mut dyn UINode,
        render_buffers: &mut UINodeRenderBuffers,
    ) {
        let texture = self
            .surface
            .get_current_texture()
            .expect("Error retrieving the current texture.");

        OrchestraRenderPipeline::draw(
            gpu_resources,
            self.size.as_(),
            &texture.texture,
            content,
            render_buffers,
        );

        texture.present();
    }

    fn get_texture_format(&self) -> TextureFormat {
        self.surface_config.format
    }
}
