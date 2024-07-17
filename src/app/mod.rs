use crate::alloc::UIFragment;
use render_state::RenderState;
use winit::{application::ApplicationHandler, event::WindowEvent};
pub mod render_state;

/// App builder, receives an UI fragment with the entirety of your app.
pub struct AppBuilder<'window, TRootFragment: UIFragment> {
    pub root_render_fragment: Option<TRootFragment>,
    event_loop: Option<winit::event_loop::EventLoop<()>>,
    running_app: Option<RunningApp<'window>>,
}

/// An app in execution (the ui fragment has been transformed into a [`RenderModule`]).
pub struct RunningApp<'window> {
    pub render_state: RenderState<'window>,
}

impl<'window, TRootFragment: UIFragment + 'static> AppBuilder<'window, TRootFragment> {
    // Creates a new app.
    // For cross-platform compatibility, this should be called in the main thread,
    // and only once in your program.
    pub fn new(root_fragment: TRootFragment) -> Self {
        let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

        Self {
            root_render_fragment: Some(root_fragment),
            event_loop: Some(event_loop),
            running_app: None,
        }
    }

    pub fn run(mut self) {
        let event_loop = self.event_loop.take().unwrap();
        event_loop.run_app(&mut self).unwrap();
    }

    fn build(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                winit::window::WindowAttributes::default()
                    .with_inner_size(winit::dpi::LogicalSize::new(640, 460))
                    .with_title("App")
                    .with_visible(true),
            )
            .unwrap();

        let root_render_fragment = self.root_render_fragment.take().unwrap();
        let render_state = RenderState::new(window, root_render_fragment);

        self.running_app = Some(RunningApp { render_state });
    }
}

impl<'window, TRootFragment: UIFragment + 'static> ApplicationHandler
    for AppBuilder<'window, TRootFragment>
{
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

impl<'window> ApplicationHandler for RunningApp<'window> {
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
                self.render_state.handle_resize(new_size);
            }
            WindowEvent::RedrawRequested => self.render_state.handle_redraw_requested(),
            _ => (),
        }
    }
}
