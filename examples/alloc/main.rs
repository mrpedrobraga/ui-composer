use cgmath::Vector2;
use futures_signals::signal::Mutable;
use std::fmt::Debug;
use ui_composer::{alloc::GBox, app::App};

trait Render: Debug {
    fn push_fragments(&self, buf: &mut Vec<Fragment>);
}

#[derive(Debug)]
enum Fragment {
    Aabb(Vector2<f32>, Vector2<f32>),
}

#[derive(Debug)]
struct Aabb(pub Vector2<f32>, pub Vector2<f32>);
impl Render for Aabb {
    fn push_fragments(&self, buf: &mut Vec<Fragment>) {
        buf.push(Fragment::Aabb(self.0, self.1));
    }
}

impl<A, B> Render for (A, B)
where
    A: Render,
    B: Render,
{
    fn push_fragments(&self, buf: &mut Vec<Fragment>) {
        self.0.push_fragments(buf);
        self.1.push_fragments(buf);
    }
}

struct Context {
    pub window_size: Mutable<Vector2<f32>>,
}

// ----- //

const fn rect(x: f32, y: f32, w: f32, h: f32) -> Aabb {
    Aabb(Vector2::new(x, y), Vector2::new(w, h))
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();

    let a = GBox::new(
        &app.render_state.unwrap().device,
        (
            rect(0.0, 0.0, 10.0, 20.0),
            (rect(0.0, 0.0, 10.0, 10.0), rect(0.0, 10.0, 10.0, 10.0)),
        ),
    );
}
