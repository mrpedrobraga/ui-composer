use cgmath::Vector2;
use futures_signals::signal::Mutable;
use slotmap::{DefaultKey, SlotMap};
use std::fmt::Debug;

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

#[derive(Debug)]
struct GPUModel {
    pub bufs: SlotMap<DefaultKey, Vec<Fragment>>,
}

struct BufferRef(DefaultKey);

impl GPUModel {
    fn new() -> Self {
        Self {
            bufs: SlotMap::new(),
        }
    }

    fn dyn_alloc<'g>(&mut self) -> BufferRef {
        BufferRef(self.bufs.insert(Vec::new()))
    }

    fn borrow_buf(&mut self, buffer_ref: BufferRef) -> Option<&mut Vec<Fragment>> {
        self.bufs.get_mut(buffer_ref.0)
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
    let mut model = GPUModel::new();

    let buf = model.dyn_alloc();
    {
        let buf = model.borrow_buf(buf).unwrap();

        let a = (
            rect(0.0, 0.0, 10.0, 20.0),
            (rect(0.0, 0.0, 10.0, 10.0), rect(0.0, 10.0, 10.0, 10.0)),
        );

        a.push_fragments(buf);
    }

    dbg!(model);
}
