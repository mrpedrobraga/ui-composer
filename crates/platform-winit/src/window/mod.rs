//! # Window
//!
//! [WindowElement]s contain several elements inside itself.
//! Every time they change (in response to an event or a future or signal yielding),
//! it will render them to its [WindowRenderTarget].

use crate::gpu::Gpu;
use crate::render::{render, RenderPipeline, RenderResources, RenderTarget, TextureSet};
use crate::runner::{WinitBlueprintResources, WinitEnvironment};
use crate::WinitUi;
use core::panic;
use futures_signals::signal::{Mutable, Signal, SignalExt as _};
use pin_project::pin_project;
use std::sync::Arc;
use std::task::Poll;
use ui_composer_core::app::composition::algebra::Bubble;
use ui_composer_core::app::composition::effects::signal::{IntoBlueprint as _, React};
use ui_composer_core::app::composition::elements::{Blueprint, Element};
use ui_composer_core::app::composition::layout::hints::ParentHints;
use ui_composer_core::app::composition::visit::DriveThru;
use ui_composer_input::event::Event;
use ui_composer_math::flow::{CartesianFlow, CurrentFlow};
use ui_composer_math::glamour::{Point2, Rect};
use ui_composer_math::prelude::Size2;
use winit::dpi::PhysicalSize;
use winit::window::{Window, WindowAttributes};

use self::effect_handling::WindowEffectVisitor;

pub mod effect_handling;

pub struct WindowBlueprint<UiBlueprint> {
    ui: UiBlueprint,
    state: WindowState,
}

pub struct WindowState {
    pub app_size: AppSize,
    pub mouse_position: Mutable<Option<Point2>>,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            app_size: Mutable::new(Mutable::new(Size2::new(640.0, 360.0))),
            mouse_position: Default::default(),
        }
    }
}

type AppSize = Mutable<Mutable<Size2>>;

pub struct WindowRuntimeState {
    pub app_size: AppSize,
    pub window_size: Mutable<Size2>,
    pub mouse_position: Mutable<Option<Point2>>,
    pub needs_redrawing: bool,
    render_resources: RenderResources,
    render_target: WindowRenderTarget,
    pub render_pipeline: RenderPipeline,
    #[allow(unused)]
    window: Arc<Window>,
}

#[allow(non_snake_case)]
pub fn Window<UiBlueprint>(
    mut ui: UiBlueprint,
) -> WindowBlueprint<React<impl Signal<Item = UiBlueprint::Blueprint>, WinitEnvironment>>
where
    UiBlueprint: WinitUi,
{
    let state = WindowState::default();
    let reshape_signal = state.app_size.signal_cloned().switch(|m| m.signal());
    let ui = reshape_signal
        .map(move |window_size| {
            let parent_hints = ParentHints {
                rect: Rect::new(Point2::ZERO, window_size),
                // TODO: Turn these into signals, maybe?
                current_flow: CurrentFlow {
                    current_flow_direction: CartesianFlow::LeftToRight,
                    current_cross_flow_direction: CartesianFlow::TopToBottom,
                    current_writing_flow_direction: CartesianFlow::LeftToRight,
                    current_writing_cross_flow_direction: CartesianFlow::TopToBottom,
                },
            };
            // TODO: Listen to and respect the child hints;
            #[allow(unused)]
            let child_hints = ui.prepare(parent_hints);
            let clamped_rect = Rect::new(
                Point2::ZERO,
                parent_hints.rect.size.max(child_hints.minimum_size),
            );
            ui.place(ParentHints {
                rect: clamped_rect,
                ..parent_hints
            })
        })
        .into_blueprint();

    WindowBlueprint { ui, state }
}

impl<UiBlueprint> Blueprint<WinitEnvironment> for WindowBlueprint<UiBlueprint>
where
    UiBlueprint: Blueprint<WinitEnvironment>,
    // for<'fx> <UiBlueprint::Element as Element<WinitEnvironment>>::Effect<'fx>: Debug,
{
    type Element = WindowElement<UiBlueprint::Element>;

    fn make(self, env: &WinitBlueprintResources<'_>) -> Self::Element {
        // TODO: Allow different attributes to be specified.
        // Ideally, the user would be able to pass `Mutable`s
        // that the window would poll for reactivity!

        let state = WindowRuntimeState::from_blueprint(self.state, env);

        WindowElement {
            ui: self.ui.make(env),
            state,
        }
    }
}

#[pin_project(project = WindowElementProj)]
pub struct WindowElement<Ui> {
    #[pin]
    ui: Ui,
    state: WindowRuntimeState,
}

impl<Ui> WindowElement<Ui> {
    fn resize_internal(&mut self, new_size: Size2) {
        self.state.resize_internal(new_size)
    }

    pub fn redraw(&mut self) {
        tracing::debug!("Redrawing!");
        self.state
            .render_resources
            .sync(&self.state.render_resources.gpu);
        let new_size = self.state.window_size.get();
        self.resize_internal(new_size);
        render(
            &self.state.render_target,
            &self.state.render_pipeline,
            &self.state.render_resources,
        );
    }
}

impl<Ui> Bubble<Event, bool> for WindowElement<Ui> {
    async fn bubble(&mut self, cx: &mut Event) -> bool {
        match cx {
            Event::Resized(_new_size) => {
                // self.resize(*new_size);
                self.state.needs_redrawing = true;
                true
            }
            Event::CloseRequested => {
                std::process::exit(1);
            }
            Event::RedrawRequested => {
                if self.state.needs_redrawing {
                    self.redraw();
                    self.state.needs_redrawing = false;
                }
                true
            }
            Event::OcclusionStateChanged(_) => false,
            Event::FocusStateChanged(_) => false,
            Event::ScaleFactorChanged(_) => false,
            Event::ThemeTypeChanged(_) => false,
            Event::Cursor { .. } => false,
            Event::Keyboard { .. } => false,
            Event::Ime(_) => false,
            Event::File(_) => false,
        }
    }
}

impl WindowRuntimeState {
    pub fn from_blueprint(blueprint: WindowState, env: &WinitBlueprintResources) -> Self {
        let window_attributes = WindowAttributes::default()
            .with_title("Hello, world!")
            .with_inner_size(PhysicalSize::new(640, 360));
        let window = env.winit_requester.request_window(window_attributes);

        let gpu = env.gpu.clone();
        let render_target = WindowRenderTarget::new(&gpu, window.clone());
        let render_pipeline = RenderPipeline::new(&gpu, wgpu::TextureFormat::Bgra8UnormSrgb);
        let render_resources = RenderResources::new(gpu, &render_pipeline);

        blueprint.app_size.set(env.window_size_mutable.clone());

        Self {
            app_size: blueprint.app_size,
            window_size: env.window_size_mutable.clone(),
            mouse_position: blueprint.mouse_position,
            render_resources,
            window,
            render_target,
            render_pipeline,
            needs_redrawing: true,
        }
    }

    pub fn resize_internal(&mut self, new_size: Size2) {
        if new_size.width == 0.0 || new_size.height == 0.0 {
            return;
        }
        self.render_resources.uniforms.resize(new_size);
        //self.app_size.set(new_size);
        self.render_target
            .resize(&self.render_resources.gpu, new_size.as_());
    }
}

impl<Ui> Element<WinitEnvironment> for WindowElement<Ui>
where
    Ui: Element<WinitEnvironment>,
    // for<'fx> Ui::Effect<'fx>: Debug
{
    type Effect<'a>
        = ()
    where
        Ui: 'a;

    fn effect(&self) -> Self::Effect<'_> {}

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
        env: &WinitBlueprintResources<'_>,
    ) -> std::task::Poll<Option<()>> {
        let WindowElementProj { mut ui, state, .. } = self.project();

        /*
            TODO: Windows will futurely have some internal state
            for their titles, size, visibility, should-close, etc.

            So we need to poll those states, too.
        */

        let inner_poll: Poll<Option<_>> = ui.as_mut().poll(cx, env);

        match inner_poll {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(_)) => {
                let quads = &mut state.render_resources.quads;
                // TODO: No cleanup will be needed when we have a sized buffer.
                quads.clear();
                let ui_effects = ui.effect();
                // dbg!(&ui_effects);
                let mut visitor = WindowEffectVisitor { quads };
                ui_effects.drive_thru(&mut visitor);
                drop(ui_effects);

                state.needs_redrawing = true;
                //state.window.request_redraw();

                Poll::Ready(Some(()))
            }
            Poll::Ready(None) => Poll::Ready(None),
        }
    }
}

/// The render target a window will draw to in order to show its elements in a [window](winit::window::Window).
pub struct WindowRenderTarget {
    pub size: Size2<u32>,
    pub surface: wgpu::Surface<'static>,
    pub depth_texture: wgpu::Texture,
}

impl WindowRenderTarget {
    /// Creates a new `RenderTarget` which renders to a window.
    pub fn new(gpu: &Gpu, window: Arc<winit::window::Window>) -> Self {
        let size = window.inner_size();
        let size = Size2::new(size.width, size.height);
        let surface = gpu
            .instance
            .create_surface(window)
            .expect("Failed to create surface for window!");
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            color_space: wgpu::SurfaceColorSpace::Auto,
        };
        surface.configure(&gpu.device, &surface_config);
        let depth_texture = Self::new_depth_texture(gpu, &size);

        Self {
            size,
            surface,
            depth_texture,
        }
    }

    fn new_depth_texture(gpu: &Gpu, size: &Size2<u32>) -> wgpu::Texture {
        gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("UI Composer Winit Window Depth Texture."),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // TODO: Maybe use ints for depth in 2D?
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
    }
}

impl RenderTarget for WindowRenderTarget {
    fn resize(&mut self, gpu: &Gpu, new_size: Size2<u32>) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: new_size.width,
            height: new_size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            color_space: wgpu::SurfaceColorSpace::Auto,
        };
        self.surface.configure(&gpu.device, &surface_config);
        self.depth_texture = Self::new_depth_texture(gpu, &new_size);
        self.size = new_size;
    }

    fn texture_set(&self) -> TextureSet {
        let surface_texture = self.surface.get_current_texture();
        match surface_texture {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => {
                let albedo = surface_texture.texture.clone();

                TextureSet {
                    surface_texture: Some(surface_texture),
                    albedo,
                    depth: self.depth_texture.clone(),
                }
            }
            _ => {
                panic!("No surface available?");
            }
        }
    }
}
