//! # Window
//!
//! [WindowElement]s contain several elements inside itself.
//! Every time they change (in response to an event or a future or signal yielding),
//! it will render them to its [WindowRenderTarget].

use crate::render::{render, RenderPipeline, RenderResources, RenderTarget as _};
use crate::runner::{WinitBlueprintResources, WinitEnvironment};
use crate::WinitUi;
use bytemuck::Zeroable;
use futures_signals::signal::Mutable;
use pin_project::pin_project;
use std::sync::Arc;
use std::task::Poll;
use ui_composer_core::app::composition::algebra::Bubble;
use ui_composer_core::app::composition::elements::{Blueprint, Element};
use ui_composer_core::app::composition::layout::hints::ParentHints;
use ui_composer_core::app::composition::visit::DriveThru as _;
use ui_composer_input::event::Event;
use ui_composer_math::flow::{CartesianFlow, CurrentFlow};
use ui_composer_math::glamour::{Point2, Rect};
use ui_composer_math::prelude::Size2;
use winit::window::Window;

use self::effect_handling::WindowEffectVisitor;

pub mod effect_handling;
pub mod render_target;

/// Describes a Window as it shall exist in the app.
pub struct WindowBlueprint<UiBlueprint> {
    ui: UiBlueprint,
    state: WindowState,
}

/// Describes the state of the window as it shall be in the app.
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

/// Function for creating a WindowBlueprint
#[allow(non_snake_case)]
pub fn Window<Ui>(ui: Ui) -> WindowBlueprint<Ui>
where
    Ui: WinitUi,
{
    let state = WindowState::default();
    WindowBlueprint { ui, state }
}

impl<Ui> Blueprint<WinitEnvironment> for WindowBlueprint<Ui>
where
    Ui: WinitUi,
    // for<'fx> <UiBlueprint::Element as Element<WinitEnvironment>>::Effect<'fx>: Debug,
{
    type Output = WindowElement<Ui>;

    fn make(self, env: &WinitBlueprintResources<'_>) -> Self::Output {
        // TODO: Allow different attributes to be specified.
        // Ideally, the user would be able to pass `Mutable`s
        // that the window would poll for reactivity!

        let state = WindowRuntimeState::from_blueprint(self.state, env);

        WindowElement { ui: self.ui, state }
    }
}

#[pin_project(project = WindowElementProj)]
pub struct WindowElement<U: WinitUi> {
    #[pin]
    ui: U,
    state: WindowRuntimeState,
}

pub struct WindowRuntimeState {
    pub app_size: AppSize,
    pub window_size: Mutable<Size2>,
    pub mouse_position: Mutable<Option<Point2>>,
    pub needs_redrawing: bool,
    render_resources: RenderResources,
    render_target: render_target::WindowRenderTarget,
    pub render_pipeline: RenderPipeline,
    #[allow(
        unused,
        reason = "It will be used in the future when we allow the user to control the window's attributes via signals."
    )]
    window: Arc<Window>,
}

impl<Ui: WinitUi> WindowElement<Ui> {
    pub(crate) fn prepare_to_resize(
        &mut self,
        new_size: Size2,
        resources: &WinitBlueprintResources,
    ) {
        /* TODO: Update the ui by telling it about the new dimensions! */
        let parent_hints = ParentHints {
            rect: Rect::new(Point2::zeroed(), new_size),
            current_flow: CurrentFlow {
                current_flow_direction: CartesianFlow::LeftToRight,
                current_cross_flow_direction: CartesianFlow::TopToBottom,
                current_writing_flow_direction: CartesianFlow::LeftToRight,
                current_writing_cross_flow_direction: CartesianFlow::TopToBottom,
            },
        };
        self.ui.place(parent_hints, resources);
    }

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

impl<Ui: WinitUi> Bubble<Event, bool> for WindowElement<Ui> {
    async fn bubble(&mut self, cx: &mut Event) -> bool {
        match cx {
            Event::Resized(new_size) => {
                self.resize_internal(*new_size);
                self.state.needs_redrawing = true;
                true
            }
            Event::CloseRequested => {
                std::process::exit(1);
            }
            Event::RedrawRequested => {
                /* TODO: Redraw! */
                if self.state.needs_redrawing {
                    let quads = &mut self.state.render_resources.quads;
                    // TODO: No cleanup will be needed when we have a sized buffer.
                    quads.clear();

                    let ui_effects = self.ui.effect();
                    let mut visitor = WindowEffectVisitor { quads };
                    ui_effects.drive_thru(&mut visitor);
                    drop(ui_effects);
                    //println!("{:?}", quads);

                    self.redraw();
                    self.state.needs_redrawing = true;
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

impl<Ui> Element<WinitEnvironment> for WindowElement<Ui>
where
    Ui: WinitUi,
{
    type Effect = ();
    type Blueprint = WindowBlueprint<Ui>;

    fn effect(&self) -> Self::Effect {}

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
        env: &WinitBlueprintResources<'_>,
    ) -> std::task::Poll<Option<()>> {
        let WindowElementProj { mut ui, state, .. } = self.project();

        /*
            TODO: Make `state` hold signals for all of a window's states
            that might change such as title, size, should_close, etc,
            poll those, and apply changes to the window as needed.
        */

        let window_size = state.window_size.get();
        let parent_hints = ParentHints {
            rect: Rect::new(Point2::zeroed(), window_size),
            current_flow: CurrentFlow {
                current_flow_direction: CartesianFlow::LeftToRight,
                current_cross_flow_direction: CartesianFlow::TopToBottom,
                current_writing_flow_direction: CartesianFlow::LeftToRight,
                current_writing_cross_flow_direction: CartesianFlow::TopToBottom,
            },
        };

        let ui_poll: Poll<Option<_>> = ui.as_mut().poll_change(cx, env, parent_hints);

        // TODO: Not sure what needs to be done
        // other than telling the window it is dirty!
        if let Poll::Ready(Some(())) = ui_poll {
            state.needs_redrawing = true;
            state.window.request_redraw();
        }

        ui_poll
    }

    fn update(&mut self, _: WindowBlueprint<Ui>, _: &WinitBlueprintResources) {
        unimplemented!()
    }
}

impl WindowRuntimeState {
    pub fn from_blueprint(blueprint: WindowState, env: &WinitBlueprintResources) -> Self {
        let gpu = env.gpu.clone();
        let window = env.window.clone().unwrap();
        let render_target = render_target::WindowRenderTarget::new(&gpu, window.clone());
        let render_pipeline = RenderPipeline::new(&gpu, wgpu::TextureFormat::Bgra8UnormSrgb);
        let render_resources = RenderResources::new(gpu, &render_pipeline);

        blueprint.app_size.set(env.window_size_mutable.clone());

        Self {
            app_size: blueprint.app_size,
            window_size: env.window_size_mutable.clone(),
            mouse_position: blueprint.mouse_position,
            render_resources,
            render_target,
            render_pipeline,
            needs_redrawing: true,
            window,
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
