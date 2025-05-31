#![allow(unused, non_snake_case)]
use ui_composer::app::input::items::Drag;
use ui_composer::items;
use ui_composer::prelude::*;
use ui_composer::wgpu::pipeline::graphics::graphic::Graphic;
use ui_composer::wgpu::pipeline::UIReifyResources;
use ui_composer::wgpu::render_target::RenderDescriptor;

fn main() {
    let window = Window(App())
        .with_title("Custom window!".into())
        .with_decorations(false);

    UIComposer::run(window);
}

fn App() -> impl LayoutItem<Content = impl RenderDescriptor> {
    let mut drag_bar = Drag();

    ResizableItem::new(move |parent| {
        drag_bar.lay(ParentHints {
            rect: Rect::new(0.0, 0.0, parent.rect.w, drag_bar.get_minimum_size().h),
            ..parent
        })
    })
    .with_minimum_size(Extent2::new(32.0, 32.0))
}

fn Drag() -> impl LayoutItem<Content = impl RenderDescriptor> {
    let s1 = Mutable::new(false);
    let s2 = Mutable::new(false);

    let factory = move |parent: ParentHints| {
        let window_drag = Drag::new(parent.rect, s1.clone(), s2.clone());

        items! {
            window_drag,
            Graphic::from(parent.rect).with_color(Rgb::new(1.0, 1.0, 1.0))
        }
    };

    ResizableItem::<_, _, UIReifyResources>::new(factory)
        .with_minimum_size(Extent2::new(10.0, 32.0))
}
