//! # Window
//!
//! [WindowElement]s contain several elements inside itself.
//! Every time they change (in response to an event or a future or signal yielding),
//! it will render them to its [WindowRenderTarget].

use crate::DesktopUi;
use crate::gpu::Gpu;
use crate::render::{RenderTarget as _};
use futures_signals::signal::Mutable;
use std::sync::Arc;
use ui_composer_core::app::composition::layout::hints::ChildHints;
use ui_composer_math::glamour::Point2;
use ui_composer_math::prelude::Size2;
use winit::dpi::PhysicalSize;
use winit::window::Window;

use self::render_target::WindowRenderTarget;

pub mod effect_handling;
pub mod render_target;

/// Describes a Window as it shall exist in the app.
pub struct WindowDescriptor<UiBlueprint> {
    pub ui: UiBlueprint,
    pub state: WindowStateDescriptor,
}

/// Describes the state of the window as it shall be in the app.
pub struct WindowStateDescriptor {
    pub app_size: Size2,
    pub mouse_position: Mutable<Option<Point2>>,
}

impl Default for WindowStateDescriptor {
    fn default() -> Self {
        Self {
            app_size: Size2::new(640.0, 360.0),
            mouse_position: Default::default(),
        }
    }
}

/// Function for creating a WindowBlueprint
#[allow(non_snake_case)]
pub fn Window<Ui>(ui: Ui) -> WindowDescriptor<Ui>
where
    Ui: DesktopUi,
{
    WindowDescriptor {
        ui,
        state: WindowStateDescriptor::default(),
    }
}

impl<Ui> WindowDescriptor<Ui>
where
    Ui: DesktopUi,
{
    pub fn initial_size(&self) -> Size2 {
        self.state.app_size
    }

    pub fn with_initial_size(mut self, size: Size2) -> Self {
        self.state.app_size = size;
        self
    }
}

pub struct WindowState {
    pub window_size: Mutable<Size2>,
    pub mouse_position: Mutable<Option<Point2>>,
    render_target: render_target::WindowRenderTarget,
    pub window: Arc<Window>,
}

impl WindowState {
    pub fn from_blueprint(
        blueprint: WindowStateDescriptor,
        gpu: Gpu,
        window: Arc<Window>,
        initial_child_hints: ChildHints,
    ) -> Self {
        let window = window.clone();
        let render_target =
            render_target::WindowRenderTarget::new(&gpu, window.clone());
        let mut window_state = Self {
            window_size: Mutable::new(blueprint.app_size),
            mouse_position: blueprint.mouse_position,
            render_target,
            window,
        };
        window_state.update_with_child_hints(initial_child_hints);
        window_state
    }

    pub fn resize(&mut self, gpu: &Gpu, new_size: Size2) {
        if new_size.width == 0.0 || new_size.height == 0.0 {
            return;
        }
        self.render_target.resize(gpu, new_size.as_());
        self.window_size.set_neq(new_size);
    }

    pub fn update_with_child_hints(&mut self, child_hints: ChildHints) {
        // TODO: See what more should follow the child hints?
        self.window.set_min_inner_size(Some(PhysicalSize {
            width: child_hints.minimum_size.width,
            height: child_hints.minimum_size.height,
        }));
    }

    pub fn render_target(&self) -> &WindowRenderTarget {
        &self.render_target
    }
}
