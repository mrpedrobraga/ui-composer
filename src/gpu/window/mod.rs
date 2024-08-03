use std::sync::Arc;

use futures_signals::signal::{Mutable, Signal, SignalExt};
use vek::{Extent2, Rect, Rgb};
use wgpu::{Surface, SurfaceConfiguration, TextureFormat, TextureView};
use winit::{
    event::WindowEvent,
    event_loop::{self, ActiveEventLoop},
    window::{Window, WindowId},
};

use super::{
    engine::{GPUResources, LiveNode, Node},
    render_target::GPURenderTarget,
    view::{View, ViewNode},
};
use crate::ui::{
    graphics::Primitive,
    layout::LayoutItem,
    node::{LiveUINode, UINode},
    react::{React, UISignalExt},
};

/// A node that describes the existence of a new window in the UI tree.
pub struct WindowNode<T: UINode> {
    attributes: WindowAttributes,
    state: WindowNodeState,
    content: T,
}

/// Creates a new window as the render target for the nodes inside.
#[allow(non_snake_case)]
pub fn Window<T>(attributes: WindowAttributes, item: T) -> WindowNode<impl UINode>
where
    T: LayoutItem + 'static,
{
    let state = WindowNodeState {
        window_size: Mutable::new(item.get_natural_size()),
    };

    let window_size_signal = state.window_size.signal();

    let minimum_size = item.get_natural_size();
    let item = state
        .window_size
        .signal()
        .map(move |window_size| item.bake(Rect::new(0.0, 0.0, window_size.w, window_size.h)))
        .into_ui();

    WindowNode {
        attributes,
        state,
        content: View(minimum_size, item).bake_react(window_size_signal),
    }
}

impl<T> Node for WindowNode<T>
where
    T: UINode + 'static,
{
    type LiveType = LiveWindowNode;

    /// Transforms a WindowNode descriptor into a live window node which can be really used!
    fn reify(self, event_loop: &ActiveEventLoop, gpu_resources: &GPUResources) -> Self::LiveType {
        let window_default_size = Extent2::new(100, 100);

        let window = event_loop
            .create_window(
                winit::window::WindowAttributes::default()
                    .with_title(self.attributes.title.get_cloned()),
            )
            .expect("Couldn't reify window!");

        let window = std::sync::Arc::new(window);

        LiveWindowNode {
            content: Box::new(self.content),
            render_target: WindowRenderTarget::new(
                &gpu_resources,
                window.clone(),
                window_default_size,
            ),
            window,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct WindowAttributes {
    pub title: Mutable<String>,
}

pub struct WindowNodeState {
    pub window_size: Mutable<Extent2<f32>>,
}

impl WindowNodeState {
    fn new(window_size: Extent2<f32>) -> Self {
        Self {
            window_size: Mutable::new(window_size),
        }
    }
}

/// A live window which contains a UI tree inside.
pub struct LiveWindowNode {
    content: Box<dyn LiveUINode>,
    render_target: WindowRenderTarget,
    window: Arc<Window>,
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
                WindowEvent::Resized(new_size) => self.render_target.resize(
                    &gpu_resources,
                    Extent2::new(new_size.width, new_size.height),
                ),
                WindowEvent::RedrawRequested => {
                    self.redraw(gpu_resources);
                }

                _ => (),
            }
        }

        self.content.handle_ui_event(event);
    }
}

const DEFAULT_CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.95,
    g: 0.95,
    b: 0.95,
    a: 1.0,
};

impl LiveWindowNode {
    fn redraw(&mut self, gpu_resources: &GPUResources) {
        self.render_target.draw(gpu_resources);
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
    }

    fn draw(&mut self, gpu_resources: &GPUResources) {
        let mut encoder =
            gpu_resources
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command Encoder"),
                });

        let texture = self
            .surface
            .get_current_texture()
            .expect("Error retrieving the current texture.");
        let view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(DEFAULT_CLEAR_COLOR),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        {
            // TODO: Render the things that go inside this render target.
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
