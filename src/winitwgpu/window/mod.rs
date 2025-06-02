use crate::app::backend::NodeReifyResources;
use crate::app::primitives::{Primitive, Processor};
use crate::layout::ParentHints;
use crate::prelude::Event;
use crate::prelude::process::React;
use crate::wgpu::backend::GPUResources;
use crate::wgpu::pipeline::graphics::RenderGraphic;
use crate::wgpu::pipeline::{
    GPURenderer,
    graphics::OrchestraRenderer,
    text::{GlyphonTextRenderer, TextPipelineBuffers},
};
use crate::wgpu::pipeline::{
    RendererBuffers, Renderers, UIReifyResources, graphics::GraphicsPipelineBuffers,
};
use crate::wgpu::render_target::{Render, RenderDescriptor, RenderTarget};
use wgpu::{
    Color, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment,
    RenderPassDescriptor, StoreOp, TextureDescriptor, TextureDimension, TextureUsages,
};
use winit::dpi::PhysicalPosition;
use {
    super::backend::{Node, NodeDescriptor},
    crate::{
        prelude::{LayoutItem, flow::CartesianFlowDirection},
        state::Mutable,
    },
    futures_signals::signal::{MutableSignalCloned, Signal, SignalExt},
    pin_project::pin_project,
    std::{
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
    },
    vek::{Extent2, Rect, Vec2},
    wgpu::{Surface, SurfaceConfiguration, TextureFormat},
    winit::{
        dpi::{LogicalSize, PhysicalSize},
        event::WindowEvent,
        event_loop::ActiveEventLoop,
        platform::x11::WindowAttributesExtX11,
        window::{Window, WindowId},
    },
};

mod conversion;

//MARK: Window Node Descriptor!
/// A node that describes the existence of a new window in the UI tree.
pub struct WindowNodeDescriptor<A> {
    state: WindowNodeState,
    content: A,
}

impl<A> WindowNodeDescriptor<A> {
    /// Consumes this window node and returns a new one with the set title.
    pub fn with_title(self, title: String) -> Self {
        let title = Mutable::new(title);

        Self {
            state: WindowNodeState {
                title_signal: title.signal_cloned(),
                ..self.state
            },
            ..self
        }
    }

    /// Adapts this window to have a reactive title — the window's title will change
    /// every time this signal changes.
    pub fn with_reactive_title(self, title: MutableSignalCloned<String>) -> Self {
        Self {
            state: WindowNodeState {
                title_signal: title,
                ..self.state
            },
            ..self
        }
    }

    /// Consumes this window node and returns a new one with the set decoration style.
    pub fn with_decorations(self, with_decorations: bool) -> Self {
        Self {
            state: WindowNodeState {
                decorations_enabled: Mutable::new(with_decorations),
                ..self.state
            },
            ..self
        }
    }
}

impl<A> NodeDescriptor for WindowNodeDescriptor<A>
where
    A: RenderDescriptor + Send + 'static,
{
    type Reified = WindowNode<A::Primitive>;

    /// Transforms a WindowNode, which merely describes a window, into an active node in an engine tree.
    fn reify(
        self,
        event_loop: &ActiveEventLoop,
        gpu_resources: &GPUResources,
        mut renderers: Renderers,
    ) -> Self::Reified {
        let window_default_size = self.state.size.get();

        assert_ne!(window_default_size.w, 0.0);
        assert_ne!(window_default_size.h, 0.0);

        let window = event_loop
            .create_window(
                winit::window::WindowAttributes::default()
                    .with_inner_size(LogicalSize::new(400.0, 400.0))
                    .with_name("UI Composer App", "UI Composer App"),
            )
            .expect("Couldn't reify window node!");

        window.set_min_inner_size(Some(PhysicalSize::new(
            window_default_size.w,
            window_default_size.h,
        )));

        let window = Arc::new(window);

        let render_buffers = RendererBuffers {
            graphics_render_buffers: GraphicsPipelineBuffers::new(
                gpu_resources,
                A::Primitive::QUAD_COUNT,
            ),
            _text_render_buffers: TextPipelineBuffers::new(
                gpu_resources,
                &mut renderers.text_renderer,
            ),
        };

        let mut reify_resources = UIReifyResources { renderers };
        let content = self.content.reify(&mut reify_resources);

        WindowNode {
            content,
            state: self.state,
            render_buffers,
            render_target: WindowRenderTarget::new(
                gpu_resources,
                window.clone(),
                window_default_size.as_(),
            ),
            window,
            reify_resources,
        }
    }
}

// MARK: Fn Constructor!
/// Describes a new window with its contents and its own state.
#[allow(non_snake_case)]
pub fn Window<A>(mut item: A) -> WindowNodeDescriptor<React<impl Signal<Item = A::Content>>>
where
    A: LayoutItem + Send + Sync,
    A::Content: RenderDescriptor,
{
    // This should be a signal that comes from the item...
    let minimum_size = item.get_natural_size();
    let minimum_size = Mutable::new(minimum_size);

    let state = WindowNodeState::new(minimum_size);

    // TODO: Make this signal change the size of the window...
    // This should probably be disallowed for targets that aren't exported to support it.
    let _window_size_signal = state.size.signal();

    // Right now items resize exclusively through their parent hints.
    let item = state.size.signal().map(move |window_size| {
        item.lay(ParentHints {
            rect: Rect::new(0.0, 0.0, window_size.w, window_size.h),
            // TODO: Allow configuring this from the locale/user settings,
            // possibly turning them into signals!
            current_flow_direction: CartesianFlowDirection::LeftToRight,
            current_cross_flow_direction: CartesianFlowDirection::TopToBottom,
            current_writing_flow_direction: CartesianFlowDirection::LeftToRight,
            current_writing_cross_flow_direction: CartesianFlowDirection::TopToBottom,
        })
    });

    WindowNodeDescriptor {
        state,
        content: React(item),
    }
}

//MARK: Attributes and state!

#[derive(Debug, Default, Clone)]
pub struct WindowAttributes<TitleSignal: Signal<Item = String>> {
    pub title: TitleSignal,
}

pub struct WindowNodeState {
    pub title_signal: MutableSignalCloned<String>,
    pub size: Mutable<Extent2<f32>>,
    pub mouse_position: Mutable<Option<Vec2<f32>>>,
    pub decorations_enabled: Mutable<bool>,
}

impl WindowNodeState {
    pub fn new(window_size: Mutable<Extent2<f32>>) -> Self {
        let title = Mutable::new(String::new());
        let title_signal = title.signal_cloned();
        let decorations_enabled = Mutable::new(true);

        WindowNodeState {
            size: window_size,
            title_signal,
            mouse_position: Mutable::new(None),
            decorations_enabled,
        }
    }
}

//MARK: Window Node!

#[pin_project(project = WindowNodeProj)]
pub struct WindowNode<R> {
    #[pin]
    state: WindowNodeState,
    window: Arc<Window>,
    content: R,
    render_buffers: RendererBuffers,
    render_target: WindowRenderTarget,
    reify_resources: UIReifyResources,
}

impl<A> Primitive<NodeReifyResources> for WindowNode<A>
where
    A: Primitive<UIReifyResources>,
{
    fn handle_event(&mut self, event: Event) -> bool {
        self.content.handle_event(event)
    }
}

impl<A> Processor<NodeReifyResources> for WindowNode<A>
where
    A: Processor<UIReifyResources>,
{
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context,
        #[expect(unused)] resources: &mut NodeReifyResources,
    ) -> Poll<Option<()>> {
        // TODO: Figure out what do to with the result of this poll (as it might introduce a need for redrawing!!!);

        let WindowNodeProj {
            content,
            window,
            mut state,
            reify_resources,
            ..
        } = self.project();

        let content = unsafe { Pin::new_unchecked(content) };
        let content_poll = content.poll(cx, reify_resources);

        if content_poll.is_ready() {
            window.request_redraw()
        }

        // Every time that a new title arrives, we update the Window!
        if let Poll::Ready(Some(new_title)) = state.title_signal.poll_change_unpin(cx) {
            window.set_title(new_title.as_str())
        }

        content_poll
    }
}

impl<R> Node for WindowNode<R>
where
    R: Render,
{
    fn setup(&mut self, _gpu_resources: &GPUResources) {}

    fn handle_window_event(
        &mut self,
        gpu_resources: &mut GPUResources,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if window_id == self.window.id() {
            match event {
                WindowEvent::Resized(new_size) => {
                    let new_size = Extent2::new(new_size.width, new_size.height);
                    self.render_target.resize(gpu_resources, new_size);
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
                    // Of course the WindowManager needs to know if the window *can* be closed - if there's any process impeding it from closing,
                    // but that's a different story.
                    std::process::exit(0);
                }
                WindowEvent::RedrawRequested => {
                    self.redraw(gpu_resources);
                }
                _ => (),
            }
        }

        if let Ok(app_event) = event.try_into() {
            self.handle_event(app_event);
        }
    }
}

impl<R> WindowNode<R>
where
    R: Render,
{
    fn redraw(&mut self, gpu_resources: &mut GPUResources) {
        self.render_target.draw(
            &mut self.content,
            gpu_resources,
            &mut self.reify_resources.renderers,
            &mut self.render_buffers,
        );
    }
}

//MARK: Window Render Target?
/// A render target that will be rendered to a window.
pub struct WindowRenderTarget {
    pub size: Extent2<u32>,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub depth_texture: wgpu::Texture,
}

impl WindowRenderTarget {
    pub fn new(gpu_resources: &GPUResources, target: Arc<Window>, size: Extent2<u32>) -> Self {
        let surface = gpu_resources.instance.create_surface(target).unwrap();
        let surface_config = surface
            .get_default_config(&gpu_resources.adapter, size.w, size.h)
            .expect("No default configuration found?");

        let depth_texture = Self::create_depth_texture(gpu_resources, size);

        Self {
            size,
            surface,
            surface_config,
            depth_texture,
        }
    }

    fn create_depth_texture(gpu_resources: &GPUResources, size: Extent2<u32>) -> wgpu::Texture {
        gpu_resources.device.create_texture(&TextureDescriptor {
            // TODO: Use better labels everywhere (possibly identifying this window).
            label: Some("Window depth texture"),
            size: wgpu::Extent3d {
                width: size.w,
                height: size.h,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            // TODO: For 2D rendering maybe I should use integers...
            // But for 3D rendering, float might be it.
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
    }
}

impl RenderTarget for WindowRenderTarget {
    fn resize(&mut self, gpu_resources: &GPUResources, new_size: Extent2<u32>) {
        self.surface_config = self
            .surface
            .get_default_config(&gpu_resources.adapter, new_size.w, new_size.h)
            .unwrap();
        self.surface
            .configure(&gpu_resources.device, &self.surface_config);
        self.depth_texture = Self::create_depth_texture(gpu_resources, new_size);
        self.size = new_size;
    }

    fn draw<'a, R: Render>(
        &mut self,
        content: &mut R,
        gpu_resources: &mut GPUResources,
        pipelines: &mut Renderers,
        render_buffers: &mut RendererBuffers,
    ) {
        let texture = self
            .surface
            .get_current_texture()
            .expect("Error retrieving the current texture.");

        let color_view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let depth_view = self
            .depth_texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            gpu_resources
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command Encoder"),
                });

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &color_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.015,
                        g: 0.015,
                        b: 0.015,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        OrchestraRenderer::draw(
            gpu_resources,
            pipelines,
            self.size.as_(),
            &texture.texture,
            &mut render_pass,
            content,
            render_buffers,
        );

        GlyphonTextRenderer::draw(
            gpu_resources,
            pipelines,
            self.size.as_(),
            &texture.texture,
            &mut render_pass,
            content,
            render_buffers,
        );

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
