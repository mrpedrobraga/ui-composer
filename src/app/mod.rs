use render_state::RenderState;
use winit::{application::ApplicationHandler, event::WindowEvent};

pub mod render_state;

pub struct App<'r> {
    pub render_state: Option<RenderState<'r>>,
}

impl<'r> App<'r> {
    // Creates a new app.
    //
    // For cross-platform compatibility, this should be called in the main thread,
    // and only once in your program.
    pub fn new() -> Self {
        App { render_state: None }
    }

    pub fn create_window(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(
                winit::window::WindowAttributes::default()
                    .with_inner_size(winit::dpi::LogicalSize::new(640, 460))
                    .with_title("Hello, World!")
                    .with_visible(true),
            )
            .unwrap();

        let render_state = RenderState::new(window);
        self.render_state = Some(render_state);
    }
}

impl<'r> ApplicationHandler for App<'r> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.create_window(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let render_state = self.render_state.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(new_size) => {
                render_state.handle_resize(new_size);
            }
            WindowEvent::RedrawRequested => render_state.handle_redraw_requested(),
            _ => (),
        }
    }
}
