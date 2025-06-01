use futures_signals::signal::Signal;
use futures_time::time::Duration;
use ui_composer::prelude::animation::*;
use ui_composer::prelude::*;
use ui_composer::state::process::React;
use ui_composer::wgpu::pipeline::text::Text;
use ui_composer::wgpu::render_target::RenderDescriptor;
use ui_composer_macros::chain;

fn main() {
    let val = Mutable::new(0.0);

    let anim = chain!({
        yield set(1.0);
        yield lerp(4.0, Duration::from_secs(4));
        yield lerp(0.0, Duration::from_secs(4));
        yield move_toward(4.0, 0.5);
    });

    let val2 = val.clone();

    std::thread::spawn(move || {
        futures::executor::block_on(anim.animate_value(val));
    });

    let app = ResizableItem::new(move |hints| reactive_text(val2.signal(), hints))
        .with_minimum_size(Extent2::new(600.0, 600.0));

    UIComposer::run(Window(app));
}

fn reactive_text<S: Signal<Item = f32> + Send>(s: S, hints: ParentHints) -> impl RenderDescriptor {
    React(s.map(move |x| {
        Text(
            hints.rect,
            format!("Value = {x:.3}"),
            Rgb::new(1.0, 1.0, 1.0),
        )
    }))
}
