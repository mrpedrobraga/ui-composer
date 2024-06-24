use futures_signals::signal::{Mutable, SignalExt};
use winit::dpi::PhysicalSize;

struct Context {
    window_size: Mutable<winit::dpi::PhysicalSize<u32>>,
}

struct Aabb(PhysicalSize<u32>);

fn main() {
    let cx = Context {
        window_size: Mutable::new(PhysicalSize::new(640, 360)),
    };

    let primitives: Vec<Aabb> = vec![];

    let fut = cx
        .window_size
        .signal()
        .for_each(|new_window_size| async move {
            dbg!(new_window_size);
        });
}
