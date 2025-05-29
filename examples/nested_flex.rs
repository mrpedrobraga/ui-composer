#![allow(non_snake_case)]

use rand::random;
use ui_composer::prelude::*;
use ui_composer::wgpu::pipeline::graphics::graphic::Graphic;
use ui_composer::wgpu::render_target::Render;
use ui_composer::Flex;

fn main() {
    let flex = Flex! ( 3;
        0.0 => Square(random_color()),
        2.0 => Flex! ( 3;
            0.0 => Square(random_color()),
            1.0 => Flex! ( 3;
                0.0 => Square(random_color()),
                1.0 => Square(random_color()),
                0.0 => Square(random_color()),
            ),
            0.0 => Square(random_color()),
        )
        .with_vertical_flow(),
        1.0 => Square(random_color()),
    );

    UIComposer::run(Window(flex).with_title("Advanced Layout".to_owned()))
}

fn Square(color: Rgb<f32>) -> impl LayoutItem<Content = impl Render> {
    ResizableItem::new(move |hx| Graphic::from(hx.rect).with_color(random_color()))
        .with_minimum_size(Extent2::new(200.0, 100.0))
}

fn random_color() -> Rgb<f32> {
    let r = random::<f32>();
    let g = random::<f32>();
    let b = random::<f32>();

    Rgb::new(r, g, b)
}
