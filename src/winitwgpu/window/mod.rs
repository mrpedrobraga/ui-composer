use {
    super::{
        backend::{Node, ReifiedNode, Resources},
        pipeline::{
            graphics::{OrchestraRenderer, RenderGraphicDescriptor},
            text::{GlyphonTextRenderer, TextPipelineBuffers},
            GPURenderer,
        },
        render_target::{Render, RenderDescriptor, RenderTarget},
    },
    crate::{
        app::node::{AppItemDescriptor, UIEvent},
        prelude::{flow::CartesianFlowDirection, LayoutItem},
        state::{
            process::{SignalProcessor, UISignalExt},
            Mutable,
        },
        ui::layout::ParentHints,
        winitwgpu::pipeline::{graphics::GraphicsPipelineBuffers, RendererBuffers, Renderers},
    },
    futures_signals::signal::{Signal, SignalExt},
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

/// A node that describes the existence of a new window in the UI tree.
pub struct WindowNodeDescriptor<A> {
    state: WindowNodeState,
    content: A,
}

impl<A> WindowNodeDescriptor<A> {
    /// Consumes this window node and returns a new one with the set title.
    pub fn with_title<Str: Into<String>>(self, title: Str) -> Self {
        Self {
            state: WindowNodeState {
                title: Mutable::new(title.into()),
                ..self.state
            },
            ..self
        }
    }

    /// Consumes this window node and returns a new one with a reactive title.
    /// The window's title will change every time this signal changes.
    pub fn with_reactive_title<Sig>(self, title_signal: Mutable<String>) -> Self {
        Self {
            state: WindowNodeState {
                title: title_signal,
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

/// Describes a new window with its contents and its own state.
#[allow(non_snake_case)]
pub fn Window<T>(
    mut item: T,
) -> WindowNodeDescriptor<SignalProcessor<impl Signal<Item = T::UIItem>, T::UIItem>>
where
    T: LayoutItem + Send + Sync,
    T::UIItem: RenderDescriptor,
{
    let minimum_size = item.get_natural_size();

    let state = WindowNodeState {
        size: Mutable::new(minimum_size),
        title: Mutable::new(String::new()),
        mouse_position: Mutable::new(None),
        decorations_enabled: Mutable::new(true),
    };

    // TODO: Make this signal change the size of the window...
    // This should probably be disallowed for targets that aren't exported to support it.
    let _window_size_signal = state.size.signal();

    // Right now items resize exclusively through their parent hints.
    let item = state
        .size
        .signal()
        .map(move |window_size| {
            item.lay(ParentHints {
                rect: Rect::new(0.0, 0.0, window_size.w, window_size.h),
                // impl RenderDescriptorTODO: Allow configuring this from the locale/user settings.
                current_flow_direction: CartesianFlowDirection::LeftToRight,
                current_cross_flow_direction: CartesianFlowDirection::TopToBottom,
                current_writing_flow_direction: CartesianFlowDirection::LeftToRight,
                current_writing_cross_flow_direction: CartesianFlowDirection::TopToBottom,
            })
        })
        .process();

    WindowNodeDescriptor {
        state,
        content: item,
    }
}

impl<A> Node for WindowNodeDescriptor<A>
where
    A: AppItemDescriptor + Render + RenderGraphicDescriptor + 'static,
{
    type Reified = WindowNode;

    /// Transforms a WindowNode, which merely describes a window, into an active node in an engine tree.
    fn reify(
        self,
        event_loop: &ActiveEventLoop,
        gpu_resources: &Resources,
        renderers: &mut Renderers,
    ) -> Self::Reified {
        let window_default_size = self.state.size.get();

        let window = event_loop
            .create_window(
                winit::window::WindowAttributes::default()
                    .with_title(self.state.title.get_cloned())
                    .with_inner_size(LogicalSize::new(400.0, 400.0))
                    .with_name("UI Composer App", "UI Composer App"),
            )
            .expect("Couldn't reify window node!");

        window.set_min_inner_size(Some(PhysicalSize::new(
            window_default_size.w,
            window_default_size.h,
        )));

        let window = std::sync::Arc::new(window);

        let render_buffers = RendererBuffers {
            graphics_render_buffers: GraphicsPipelineBuffers::new(gpu_resources, A::QUAD_COUNT),
            text_render_buffers: TextPipelineBuffers::new(
                gpu_resources,
                &mut renderers.text_renderer,
            ),
        };

        WindowNode {
            content: Box::new(self.content),
            state: self.state,
            render_buffers,
            render_target: WindowRenderTarget::new(
                gpu_resources,
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
    pub mouse_position: Mutable<Option<Vec2<f32>>>,
    pub decorations_enabled: Mutable<bool>,
}

impl WindowNodeState {
    pub fn new(window_size: Mutable<Extent2<f32>>) -> Self {
        WindowNodeState {
            size: window_size,
            title: Mutable::new(String::new()),
            mouse_position: Mutable::new(None),
            decorations_enabled: Mutable::new(true),
        }
    }
}
/// A live window which contains a UI tree inside.
#[pin_project(project = WindowNodeProj)]
pub struct WindowNode {
    #[pin]
    state: WindowNodeState,
    window: Arc<Window>,
    content: Box<dyn Render>,
    render_buffers: RendererBuffers,
    render_target: WindowRenderTarget,
}

impl ReifiedNode for WindowNode {
    fn setup(&mut self, _gpu_resources: &Resources) {}

    fn handle_window_event(
        &mut self,
        gpu_resources: &mut Resources,
        pipelines: &mut Renderers,
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
                    self.redraw(gpu_resources, pipelines);
                }
                _ => (),
            }
        }

        self.content.handle_ui_event(UIEvent::default());
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        // TODO: Figure out what do to with the result of this poll (as it might introduce a need for redrawing!!!);

        let WindowNodeProj {
            content, window, ..
        } = self.project();

        let content: &mut _ = &mut **content;
        let content = unsafe { Pin::new_unchecked(content) };

        let poll = content.poll_processors(cx);

        if poll.is_ready() {
            window.request_redraw()
        }

        poll
    }
}

impl WindowNode {
    fn redraw(&mut self, gpu_resources: &mut Resources, pipelines: &mut Renderers) {
        self.render_target.draw(
            self.content.as_mut(),
            gpu_resources,
            pipelines,
            &mut self.render_buffers,
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
    pub fn new(gpu_resources: &Resources, target: Arc<Window>, size: Extent2<u32>) -> Self {
        let surface = gpu_resources.instance.create_surface(target).unwrap();
        let surface_config = surface
            .get_default_config(&gpu_resources.adapter, size.w, size.h)
            .expect("No default configuration found?");

        Self {
            size,
            surface,
            surface_config,
        }
    }
}

impl RenderTarget for WindowRenderTarget {
    fn resize(&mut self, gpu_resources: &Resources, new_size: Extent2<u32>) {
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
        content: &mut dyn Render,
        gpu_resources: &mut Resources,
        pipelines: &mut Renderers,
        render_buffers: &mut RendererBuffers,
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
                        r: 0.015,
                        g: 0.015,
                        b: 0.015,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
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
