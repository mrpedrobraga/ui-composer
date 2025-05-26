#![allow(non_snake_case)]

use ui_composer::gpu::pipeline::text::Text;
use ui_composer::prelude::flow::CartesianFlowDirection;
use ui_composer::Flex;
use ui_composer::prelude::*;

fn main() {
    let flex = Flex! {
        0.0 => Square(Rgb::new(126.0, 46.0, 132.0) / 255.0),
        2.0 => Flex! {
            0.0 => Label(String::from("Sample Text"), Rgb::new(209.0, 64.0, 129.0) / 255.0),
            0.0 => Square(Rgb::new(209.0, 64.0, 129.0) / 255.0),
            0.0 => Square(Rgb::new(249.0, 245.0, 227.0) / 255.0)
        }.with_flow(CartesianFlowDirection::TopToBottom.into()),
        1.0 => Square(Rgb::new(239.0, 121.0, 138.0) / 255.0),
    };

    UIComposer::run(Window(flex).with_title("Advanced Layout"))
}

fn Square(color: Rgb<f32>) -> impl LayoutItem {
    ResizableItem::new(move |hx| {
        hx.rect
            .with_color(color)
    })
    .with_minimum_size(Extent2::new(100.0, 300.0))
}

fn Label(text: String, color: Rgb<f32>) -> impl LayoutItem {
    ResizableItem::new(move |hx| {
        Text(hx.rect.expand_from_center(-8.0, -8.0, -8.0, -8.0), text.clone(), color)
    }).with_minimum_size(Extent2::new(32.0, 48.0))
}