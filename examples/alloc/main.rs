use bytemuck::NoUninit;
use futures_signals::signal::Mutable;
use std::fmt::Debug;
use ui_composer::{
    alloc::{GBox, IntoGPU},
    app::App,
};

trait Render: Debug {
    fn push_fragments(&self, buf: &mut Vec<Fragment>);
}

#[derive(Clone)]
struct W<T: Render>(pub T);

impl<T> IntoGPU for W<T>
where
    T: Render,
{
    fn push_bytes<'slice>(self, bytes: &mut Vec<u8>) {
        let mut v = Vec::new();
        self.0.push_fragments(&mut v);
        bytes.extend(bytemuck::cast_slice(v.as_slice()).iter());
    }
}

#[derive(Debug, Clone, Copy, NoUninit)]
#[repr(C)]
struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, NoUninit, Clone, Copy)]
#[repr(C)]
struct Fragment {
    position: Vector2,
    size: Vector2,
}

impl IntoGPU for Fragment {
    fn push_bytes<'slice>(self, bytes: &mut Vec<u8>) {
        bytes.extend(bytemuck::cast_slice(&[self]));
    }
}

#[derive(Debug, NoUninit, Clone, Copy)]
#[repr(C)]
struct Aabb(pub Vector2, pub Vector2);
impl Render for Aabb {
    fn push_fragments(&self, buf: &mut Vec<Fragment>) {
        buf.push(Fragment {
            position: self.0,
            size: self.1,
        });
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
    pub window_size: Mutable<Vector2>,
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
        W((
            rect(0.0, 0.0, 1.0, 1.0),
            (rect(0.0, 0.0, 6.0, 3.0), rect(0.0, 3.0, 6.0, 3.0)),
        )),
    );
}
