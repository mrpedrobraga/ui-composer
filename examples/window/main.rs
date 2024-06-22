use ui_composer::app::App;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::builder().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
