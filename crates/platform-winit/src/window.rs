//! # Window
//!
//! [WindowElement]s contain several elements inside itself.
//! Every time they change (in response to an event or a future or signal yielding),
//! it will render them to its [WindowRenderTarget].

use pin_project::pin_project;
use ui_composer_input::event::Event;
use ui_composer_math::prelude::Size2;
use winit::dpi::PhysicalSize;
use winit::window::{Window, WindowAttributes};

use crate::gpu::{Gpu, RenderTarget};
use crate::runner::{WinitBlueprintResources, WinitEnvironment};
use std::sync::Arc;
use std::task::Poll;
use ui_composer_core::app::composition::algebra::Bubble;
use ui_composer_core::app::composition::elements::{Blueprint, Element};

pub struct WindowBlueprint<UiBlueprint> {
    ui: UiBlueprint,
}

pub fn window<A>(ui: A) -> WindowBlueprint<A> {
    WindowBlueprint { ui }
}

impl<UiBlueprint> Blueprint<WinitEnvironment> for WindowBlueprint<UiBlueprint>
where
    UiBlueprint: Blueprint<WinitEnvironment>,
{
    type Element = WindowElement<UiBlueprint::Element>;

    fn make(self, env: &WinitBlueprintResources<'_>) -> Self::Element {
        // TODO: Allow different attributes to be specified.
        // Ideally, the user would be able to pass `Mutable`s
        // that the window would poll for reactivity!
        let window_attributes = WindowAttributes::default()
            .with_title("Hello, world!")
            .with_inner_size(PhysicalSize::new(300, 300));
        let window = env.winit_requester.request_window(window_attributes);

        WindowElement {
            ui: self.ui.make(env),
            window,
        }
    }
}

#[pin_project(project = WindowElementProj)]
pub struct WindowElement<Ui> {
    #[pin]
    ui: Ui,
    window: Arc<Window>,
}

impl<Ui> Bubble<Event, bool> for WindowElement<Ui> {
    fn bubble(&mut self, cx: &mut Event) -> bool {
        match cx {
            Event::Resized(_extent2) => {
                /* Store and broadcast this change by setting the window's state. */
                true
            }
            Event::CloseRequested => false,
            Event::RedrawRequested => false,
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

impl<Ui> Element<WinitEnvironment> for WindowElement<Ui>
where
    Ui: Element<WinitEnvironment>,
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
        let WindowElementProj { ui, window } = self.project();

        /*
            TODO: Windows will futurely have some internal state
            for their titles, size, visibility, should-close, etc.

            So we need to poll those states, too.
        */

        let inner_poll: Poll<Option<_>> = ui.poll(cx, env);

        match inner_poll {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(_)) => {
                /*
                    TODO: Actually draw stuff to the window.
                */
                window.request_redraw();

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
        let surface = wgpu::Instance::default()
            .create_surface(window)
            .expect("Failed to create surface for window!");
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
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
    }
}

impl RenderTarget for WindowRenderTarget {
    async fn resize(&mut self, gpu: &Gpu, new_size: Size2<u32>) {
        let adapter = wgpu::Instance::default()
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&self.surface),
                ..Default::default()
            })
            .await
            .expect("Failed to request new adapter!");
        let surface_config = self
            .surface
            .get_default_config(&adapter, new_size.width, new_size.height)
            .expect("Failed to get new configuration for surface.");
        self.surface.configure(&gpu.device, &surface_config);
        self.depth_texture = Self::new_depth_texture(gpu, &new_size);
        self.size = new_size;
    }
}
