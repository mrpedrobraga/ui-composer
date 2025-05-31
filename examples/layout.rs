#![allow(non_snake_case)]

use ui_composer::prelude::*;
use ui_composer::wgpu::pipeline::graphics::graphic::Graphic;
use ui_composer::wgpu::pipeline::UIReifyResources;
use ui_composer::wgpu::render_target::RenderDescriptor;
use ui_composer::Flex2;

fn main() {
    let grape = Rgb::new(126.0, 46.0, 132.0) / 255.0;
    let dragonfruit = Rgb::new(209.0, 64.0, 129.0) / 255.0;
    let vanilla = Rgb::new(249.0, 245.0, 227.0) / 255.0;
    let peach = Rgb::new(239.0, 121.0, 138.0) / 255.0;

    let flex = Flex2! ( 3;
        [_] Square(grape),
        [2.0] Flex2! ( 3;
            [_] Square(dragonfruit),
            [3.0] Square(vanilla),
            [2.0] Square(dragonfruit),
        )
        .with_vertical_flow(),
        [_] Square(peach),
    );

    UIComposer::run(Window(flex).with_title("Advanced Layout".to_owned()))
}

fn Square(color: Rgb<f32>) -> impl LayoutItem<Content = impl RenderDescriptor> {
    ResizableItem::<_, _, UIReifyResources>::new(move |hx| Graphic::from(hx.rect).with_color(color))
        .with_minimum_size(Extent2::new(200.0, 100.0))
}
