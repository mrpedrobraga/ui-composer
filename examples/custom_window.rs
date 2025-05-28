#![allow(unused, non_snake_case)]
use ui_composer::app::input::items::Drag;
use ui_composer::items;
use ui_composer::prelude::*;
use ui_composer::wgpu::pipeline::graphics::graphic::Graphic;
use ui_composer::wgpu::render_target::Render;

fn main() {
    let window = Window(App())
        .with_title("Custom window!".into())
        .with_decorations(false);

    UIComposer::run(window);
}

fn App() -> impl LayoutItem<Content = impl Render> {
    let mut drag_bar = Drag();

    ResizableItem::new(move |parent| {
        drag_bar.lay(ParentHints {
            rect: Rect::new(0.0, 0.0, parent.rect.w, drag_bar.get_minimum_size().h),
            ..parent
        })
    })
}

fn Drag() -> impl LayoutItem<Content = impl Render> {
    let s1 = Mutable::new(false);
    let s2 = Mutable::new(false);

    ResizableItem::new(move |parent| {
        let window_drag = Drag::new(parent.rect, s1.clone(), s2.clone());

        items! {
            window_drag,
            Graphic::from(parent.rect).with_color(Rgb::new(1.0, 1.0, 1.0))
        }
    })
    .with_minimum_size(Extent2::new(0.0, 32.0))
}
