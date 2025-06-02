use futures_time::time::Duration;
use ui_composer::prelude::animation::{lerp, set};
use ui_composer::prelude::*;
use ui_composer::state::animation::RealTimeStream;
use ui_composer::state::process::{Await, React};
use ui_composer::wgpu::pipeline::UIReifyResources;
use ui_composer::wgpu::pipeline::graphics::graphic::Graphic;
use ui_composer::wgpu::render_target::RenderDescriptor;
use ui_composer_macros::chain;

fn main() {
    UIComposer::run(Window(App()));
}

#[allow(non_snake_case)]
fn App() -> impl LayoutItem<Content = impl RenderDescriptor> {
    let animation_state = Mutable::new(3.0);

    ResizableItem::<_, _, UIReifyResources>::new(move |parent| {
        let animated_square = animation_state.signal().map(move |x| {
            Graphic::from(Rect::new(x, parent.rect.h / 2.0, 32.0, 32.0))
                .with_color(Rgb::yellow())
                .rotated(x / 100.0)
        });

        let animation = chain!({
            yield set(0.0);
            yield lerp(500.0, Duration::from_secs(3));
            yield lerp(0.00, Duration::from_secs(1));
        })
        .animate_value(animation_state.clone());

        (React(animated_square), Await(animation))
    })
    .with_minimum_size(Extent2::new(600.0, 600.0))
}
