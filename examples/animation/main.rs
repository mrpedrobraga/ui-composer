use futures_time::time::Duration;
use ui_composer::prelude::*;
use ui_composer::state::animation::{InitialValue, RealTimeStream};
use ui_composer::wgpu::pipeline::graphics::graphic::Graphic;
use ui_composer::UI;

fn main() {
    UIComposer::run(Window(App()));
}

#[allow(non_snake_case)]
fn App() -> UI!() {
    let animation_state = Mutable::new(3.0);

    ResizableItem::new(move |parent| {
        let animated_square = animation_state.signal().map(move |x| {
            Graphic::from(Rect::new(x, parent.rect.h / 2.0, 32.0, 32.0))
                .with_color(Rgb::yellow())
                .rotated(x / 10.0)
        });

        // To Do -- Currently I'm regenerating this future
        // at every layout shift, which causes data races.
        //
        // It should be possible to hold processes as "LayoutItem" instead of "Primitive"
        let animation = back_and_forth(animation_state.clone());

        (animated_square.process(), animation.process())
    })
}

fn back_and_forth<S>(slot: S) -> impl std::future::Future<Output = ()>
where
    S: Slot<Item = f32> + 'static,
{
    InitialValue(0.0)
        .lerp_to(500.0, Duration::from_secs(3))
        .lerp_to(0.0, Duration::from_secs(1))
        .animate_value(slot)
}
