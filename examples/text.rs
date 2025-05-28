#![allow(non_snake_case)]
use ui_composer::wgpu::pipeline::{graphics::graphic::Graphic, text::Text};
use ui_composer::wgpu::render_target::Render;
use ui_composer::{items, prelude::*};

fn main() {
    UIComposer::run(Window(App()));
}

fn App() -> impl LayoutItem<Content = impl Render> {
    Center(Row(
        Square(Rgb::new(0.6, 0.7, 0.8)),
        Square(Rgb::new(0.7, 0.8, 0.6)),
    ))
}

fn Square(color: Rgb<f32>) -> impl LayoutItem<Content = impl Render> {
    ResizableItem::new(move |parent| {
        items!(
            Graphic::new(parent.rect, color),
            Text(parent.rect, String::from("Hello!"), Rgb::new(1.0, 1.0, 1.0))
        )
    })
    .with_minimum_size(Extent2::new(100.0, 100.0))
}
