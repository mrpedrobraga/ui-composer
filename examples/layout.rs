#![allow(non_snake_case)]

use ui_composer::prelude::*;
use ui_composer::winitwgpu::pipeline::graphics::graphic::Graphic;
use ui_composer::Flex;

fn main() {
    let grape = Rgb::new(126.0, 46.0, 132.0) / 255.0;
    let dragonfruit = Rgb::new(209.0, 64.0, 129.0) / 255.0;
    let vanilla = Rgb::new(249.0, 245.0, 227.0) / 255.0;
    let peach = Rgb::new(239.0, 121.0, 138.0) / 255.0;

    let flex = Flex! (
        0.0 => Square(grape),
        2.0 => Flex! (
            0.0 => Label(String::from("Sample Text")).with_color(dragonfruit),
            1.0 => Square(vanilla),
            0.0 => Square(dragonfruit)
        )
        .with_vertical_flow(),
        1.0 => Square(peach),
    );

    UIComposer::run(Window(flex).with_title("Advanced Layout"))
}

fn Square(color: Rgb<f32>) -> impl LayoutItem {
    ResizableItem::new(move |hx| Graphic::from(hx.rect).with_color(color))
        .with_minimum_size(Extent2::new(200.0, 100.0))
}
