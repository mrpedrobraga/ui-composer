#![allow(non_snake_case)]

use ui_composer::items;
use ui_composer::prelude::animation::spring::Spring;
use ui_composer::prelude::*;

fn main() {
    let flex = Flex(items!(
        FlexItem(
            Square(Rgb::new(255.0 / 255.0, 199.0 / 255.0, 95.0 / 255.0)),
            0.0,
        ),
        FlexItem(
            Square(Rgb::new(255.0 / 255.0, 111.0 / 255.0, 145.0 / 255.0)),
            2.0,
        ),
        FlexItem(
            Square(Rgb::new(255.0 / 255.0, 199.0 / 255.0, 95.0 / 255.0)),
            1.0,
        ),
    ));

    App::run(Window(flex).with_title("Flex Example"))
}

fn Square(color: Rgb<f32>) -> impl LayoutItem {
    let hover_state = Editable::new(false);
    let anim_state = Editable::new(0.0);

    Resizable::new(move |hx| {
        let hover_state = hover_state.clone();
        items!(
            Hover::new(hx.rect, hover_state.clone()),
            Spring::if_then_else(hover_state.signal(), anim_state.clone(), 1.0, 0.0).process(),
            anim_state
                .signal()
                .map(move |anim_factor| { hx.rect.expand(anim_factor * -10.0).with_color(color) })
                .process()
        )
    })
    .with_minimum_size(Extent2::new(100.0, 600.0))
}
