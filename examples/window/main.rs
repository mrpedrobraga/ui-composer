use ui_composer::{alloc::GBox, app::App};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::builder().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new();

    // Creates a GPU Box (a heap allocation on the cpu);
    //
    // Beyond this you can imagine pseudo types like GVec, GTuple, GEnum, etc.
    let buf = GBox::new(&app.render_state.as_ref().unwrap().device, (10, 20));

    event_loop.run_app(&mut app).unwrap();
}
