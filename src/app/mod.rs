use std::sync::{Arc, Mutex};

use crate::render_module::IntoRenderModule;
use engine::UIEngine;
use winit::{
    application::ApplicationHandler, event::WindowEvent,
    platform::wayland::WindowAttributesExtWayland, window::WindowAttributes,
};
pub mod engine;

/// App builder, receives an UI fragment with the entirety of your app.
pub struct AppBuilder<I: IntoRenderModule> {
    root_render_fragment: Option<I>,
    event_loop: Option<winit::event_loop::EventLoop<()>>,
    window: Option<winit::window::Window>,
    window_attributes: Option<WindowAttributes>,
    running_app: Option<RunningApp>,
}

/// An app in execution (the ui fragment has been transformed into a [`RenderModule`]).
pub struct RunningApp {
    engine: Arc<Mutex<UIEngine>>,
}

/// TODO: PRovide methods to bind to an existing Event Loop or window.
impl<I: IntoRenderModule> AppBuilder<I> {
    // Creates a new app.
    // For cross-platform compatibility, this should be called in the main thread,
    // and only once in your program.
    pub fn new(root_fragment: I) -> Self {
        Self {
            root_render_fragment: Some(root_fragment),
            event_loop: None,
            window: None,
            running_app: None,
            window_attributes: None,
        }
    }

    pub fn with_window_attributes(
        mut self,
        window_attributes: winit::window::WindowAttributes,
    ) -> Self {
        self.window_attributes = Some(window_attributes);
        self
    }

    pub fn run(mut self) {
        let event_loop = self.event_loop.take().unwrap_or_else(|| {
            let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
            event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
            event_loop
        });
        event_loop.run_app(&mut self).unwrap();
    }

    fn build(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = self.window.take().unwrap_or_else(|| {
            event_loop
                .create_window(self.window_attributes.take().unwrap_or_else(|| {
                    winit::window::WindowAttributes::default()
                        .with_inner_size(winit::dpi::LogicalSize::new(640, 640))
                        .with_title("UI Composer App")
                        .with_name("UI Composer App", "")
                        .with_visible(true)
                }))
                .unwrap()
        });

        let root_render_fragment = self.root_render_fragment.take().unwrap();
        let render_state = UIEngine::new(window, root_render_fragment);

        self.running_app = Some(RunningApp {
            engine: render_state,
        });
    }
}

impl<I: IntoRenderModule> ApplicationHandler for AppBuilder<I> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.build(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(running_app) = &mut self.running_app {
            running_app.window_event(event_loop, window_id, event)
        }
    }
}

impl ApplicationHandler for RunningApp {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(new_size) => {
                let mut engine = self
                    .engine
                    .lock()
                    .expect("Could not lock Render Engine to resize");
                engine.handle_resize(new_size);
            }
            WindowEvent::RedrawRequested => {
                let mut engine = self
                    .engine
                    .lock()
                    .expect("Could not lock Render Engine to request redraw");
                engine.handle_redraw_requested()
            }
            _ => (),
        }

        let mut engine = self
            .engine
            .lock()
            .expect("Could not lock Render Engine to pump window event");
        engine.handle_window_event(event);
    }
}
