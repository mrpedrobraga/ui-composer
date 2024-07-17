#![allow(non_snake_case)]
use ui_composer::{alloc::UIFragment, app::AppBuilder, fragments::Primitive};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::builder().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app_builder = AppBuilder::new(MyApp());
    event_loop.run_app(&mut app_builder).unwrap();
}

fn MyApp() -> impl UIFragment {
    Primitive {
        color: [0.5, 0.5, 1.0],
    }
}
