use std::{
    iter,
    thread::{self, sleep},
    time::Duration,
};

use futures_signals::signal::SignalExt as _;
use ui_composer::{
    interaction::{hover::HoverInteraction, InteractorNode},
    prelude::AABB,
    reaction::PrimitiveSpliceReactor,
    standard::primitive::Primitive,
};
use winit::event::{DeviceId, WindowEvent};

fn main() {
    let aabb = AABB::new((0, 0), (10, 10));

    let mut hover = HoverInteraction::new(aabb);
    let primitive = hover.get_signal().map(|foo| {
        Rect(
            aabb,
            if foo {
                (1.0, 0.0, 0.0)
            } else {
                (0.0, 1.0, 1.0)
            },
        )
    });

    let r = PrimitiveSpliceReactor::new(primitive.map(|pri| iter::once(pri)), 0..1);

    thread::spawn(move || {
        sleep(Duration::from_secs(1));
        hover.handle_event(WindowEvent::CursorMoved {
            device_id: unsafe { DeviceId::dummy() },
            position: winit::dpi::PhysicalPosition { x: 20.0, y: 20.0 },
        });
        sleep(Duration::from_secs(1));
        hover.handle_event(WindowEvent::CursorMoved {
            device_id: unsafe { DeviceId::dummy() },
            position: winit::dpi::PhysicalPosition { x: 5.0, y: 5.0 },
        });
    });

    pollster::block_on(r.for_each(|splice| async {
        dbg!(splice.collect::<Vec<_>>());
    }));
}

fn Rect(aabb: AABB, color: (f32, f32, f32)) -> Primitive {
    Primitive {
        transform: [
            [aabb.size.0 as f32, 0.0, 0.0, 0.0],
            [0.0, aabb.size.1 as f32, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [aabb.top_left.0 as f32, aabb.top_left.1 as f32, 0.0, 1.0],
        ],
        color: [color.0, color.1, color.2],
    }
}
